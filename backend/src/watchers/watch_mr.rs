use helpers::{LogError, Notifier};
use managers::GitlabManager;
use managers::gitlab::structs::{GlMergeRequest, GlMergeRequestState};
use rusqlite::Connection;
use std::path::PathBuf;

struct WatchMRChainMrTask {
    pub id: i32,
    pub source_branch: String,
    pub target_branch: String,
}

struct WatchMRResult {
    pub id: i32,
    pub mr_id: i32,
    pub project_id: i32,
    pub has_conflicts: bool,

    pub notify_on_end: bool,
    pub auto_merge: bool,
    pub watch_pipeline_after_merge: bool,

    pub chainmr_task: Option<WatchMRChainMrTask>,
}

impl WatchMRResult {
    pub fn get_title(&self) -> String {
        match &self.chainmr_task {
            None => {
                format!("MR !{}", self.mr_id)
            }
            Some(task) => {
                format!(
                    "#{}: {} -> {}",
                    task.id, task.source_branch, task.target_branch,
                )
            }
        }
    }
}

pub fn watch_mrs(
    db_path: &PathBuf,
    gitlab_manager: &GitlabManager,
) -> Result<(), String> {
    let conn = Connection::open(db_path).log_error()?;

    let watch_mrs = get_watch_mrs(&conn)?;

    for watch_mr in watch_mrs {
        let mr = match gitlab_manager
            .get_merge_request(
                watch_mr.project_id as u64,
                watch_mr.mr_id as u64,
            )
            .log_error()
        {
            Ok(mr) => mr,
            Err(_) => continue,
        };

        update_watch_mr(&conn, &watch_mr, &mr);

        if (mr.has_conflicts && !watch_mr.has_conflicts) {
            Notifier::notify(
                watch_mr.get_title(),
                Some("Обнаружен конфликт. Необходим ручной мердж".to_string()),
                vec![(mr.web_url, "Открыть MR".to_string())],
                |url| {
                    let _ = open::that(url).log_error();
                },
            );

            continue;
        }

        match mr.head_pipeline {
            None => {}
            Some(pipeline) => {
                if pipeline.status.is_failed() {
                    Notifier::notify(
                        watch_mr.get_title(),
                        Some("Упал пайплайн!".to_string()),
                        vec![
                            (mr.web_url, "Открыть MR".to_string()),
                            (pipeline.web_url, "Открыть пайплайн".to_string()),
                        ],
                        |url| {
                            let _ = open::that(url).log_error();
                        },
                    );

                    continue;
                }

                if !pipeline.status.is_success() {
                    continue;
                }
            }
        }

        if mr.has_conflicts {
            continue;
        }

        match mr.state {
            GlMergeRequestState::Closed => {
                if !watch_mr.notify_on_end {
                    continue
                }
                
                Notifier::notify(
                    watch_mr.get_title(),
                    Some("MR закрыт!".to_string()),
                    vec![(mr.web_url, "Открыть MR".to_string())],
                    |url| {
                        let _ = open::that(url).log_error();
                    },
                );

                continue;
            }
            _ => {}
        }

        if !watch_mr.auto_merge {
            Notifier::notify(
                watch_mr.get_title(),
                Some(format!("MR {} готов к мерджу!", mr.id)),
                vec![(mr.web_url, "Открыть MR".to_string())],
                |url| {
                    let _ = open::that(url).log_error();
                },
            );

            continue;
        }

        match gitlab_manager
            .merge_mr(watch_mr.project_id as u64, watch_mr.mr_id as u64)
        {
            Ok(merged_mr) => {
                let _ = conn.execute(
                    "
                update watch__mr
                set merge_commit_sha = ?1,
                    status = 'merged'
                where id = ?2",
                    (merged_mr.merge_commit_sha, watch_mr.id),
                );

                if watch_mr.notify_on_end {
                    Notifier::notify(
                        watch_mr.get_title(),
                        Some(format!("MR {} смерджен!", mr.id)),
                        vec![(mr.web_url, "Открыть MR".to_string())],
                        |url| {
                            let _ = open::that(url).log_error();
                        },
                    )
                }
            }
            Err(err) => Notifier::notify(
                watch_mr.get_title(),
                Some(format!("Ошибка мерджа MR: {}", err)),
                vec![(mr.web_url, "Открыть MR".to_string())],
                |url| {
                    let _ = open::that(url).log_error();
                },
            ),
        }
    }

    Ok(())
}

fn get_watch_mrs(conn: &Connection) -> Result<Vec<WatchMRResult>, String> {
    let mut mr_query = conn
        .prepare(
            "select wmr.id,
                wmr.mr_id,
                wmr.project_id,
                wmr.notify_on_end,
                wmr.auto_merge,
                wmr.watch_pipline_after_merge,
                ct.id,
                ct.source_branch,
                ct.target_branch,
                wmr.has_conflicts
                from watch__mr wmr
                    left join main.chainmr__step cs on wmr.id = cs.watch_mr_id
                    left outer join main.chainmr__task ct on ct.id = cs.task_id
                where wmr.status = 'opened'
        ",
        )
        .log_error()?;

    let merge_requests = mr_query
        .query_map([], |row| {
            let chainmr_task_id: Option<i32> = row.get(6).log_error().unwrap();

            Ok(WatchMRResult {
                id: row.get(0).log_error().unwrap(),
                mr_id: row.get(1).log_error().unwrap(),
                project_id: row.get(2).log_error().unwrap(),
                notify_on_end: row.get(3).log_error().unwrap(),
                auto_merge: row.get(4).log_error().unwrap(),
                watch_pipeline_after_merge: row.get(5).log_error().unwrap(),
                chainmr_task: match chainmr_task_id {
                    None => None,
                    Some(id) => Some(WatchMRChainMrTask {
                        id,
                        source_branch: row.get(7).log_error().unwrap(),
                        target_branch: row.get(8).log_error().unwrap(),
                    }),
                },
                has_conflicts: row.get(9).log_error().unwrap(),
            })
        })
        .log_error()?;

    Ok(merge_requests.map(|i| i.log_error().unwrap()).collect())
}

fn update_watch_mr(
    conn: &Connection,
    watch_mr: &WatchMRResult,
    mr: &GlMergeRequest,
) {
    let res = conn
        .execute(
            "update watch__mr set
                     status = ?1,
                     has_conflicts = ?2
            where id = ?3",
            (mr.state.to_string(), mr.has_conflicts, watch_mr.id),
        )
        .log_error();
}
