use helpers::{Branch, LogError, Notifier};
use managers::GitlabManager;
use managers::gitlab::structs::{GlMergeRequest, GlMergeRequestState};
use regex::Regex;
use rusqlite::{Connection, params};
use std::path::PathBuf;

struct ChainmrTaskResult {
    id: i32,
    project_id: i32,
    source_branch: String,
    target_branch: String,
}

struct ChainmrStepResult {
    id: i32,
    step_number: i32,
    source_branch: Branch,
    target_branch: Branch,
    steps_count: i32,
    mr_status: Option<GlMergeRequestState>,
    mr_web_url: Option<String>,
    mr_id: Option<i32>,
    task: ChainmrTaskResult,
}

impl ChainmrStepResult {
    pub fn get_title(&self) -> String {
        format!(
            "#{}: {} -> {}",
            self.task.id, self.task.source_branch, self.task.target_branch
        )
    }
}

pub fn watch_chainmr(
    db_path: &PathBuf,
    gitlab_manager: &GitlabManager,
) -> Result<(), String> {
    let conn = Connection::open(db_path).log_error()?;

    let steps = load_steps(&conn)?;

    for step in steps {
        match step.mr_status {
            Some(GlMergeRequestState::Closed) => on_close_mr(&step, &conn)?,
            Some(GlMergeRequestState::Merged) => on_merge_mr(&step, &conn)?,
            None => start_task(&step, &conn, gitlab_manager)?,
            _ => {}
        }
    }

    Ok(())
}

fn load_steps(conn: &Connection) -> Result<Vec<ChainmrStepResult>, String> {
    let mut steps_query = conn
        .prepare(
            "\
        select cms.id,
               cms.step_number,
               cms.source_branch,
               cms.target_branch,
               cmt.steps_count,
               wm.status,
               cmt.id as cmt_id,
               cmt.project_id,
               cmt.source_branch as cmt_source_branch,
               cmt.target_branch as cmt_target_branch,
               wm.web_url,
               wm.mr_id
        from chainmr__step cms
                 join chainmr__task cmt on cmt.id = cms.task_id
                 left join main.watch__mr wm on cms.watch_mr_id = wm.id
        where cms.status = 'pending'
          and cmt.status = 'pending'
    ",
        )
        .log_error()?;

    let steps = steps_query
        .query_map([], |row| {
            let status: Option<String> = row.get("status")?;
            Ok(ChainmrStepResult {
                id: row.get("id")?,
                step_number: row.get("step_number")?,
                source_branch: row.get("source_branch")?,
                target_branch: row.get("target_branch")?,
                steps_count: row.get("steps_count")?,
                mr_status: match status {
                    None => None,
                    Some(status) => Some(GlMergeRequestState::from(status)),
                },
                mr_web_url: row.get("web_url")?,
                mr_id: row.get("mr_id")?,
                task: ChainmrTaskResult {
                    id: row.get("cmt_id")?,
                    project_id: row.get("project_id")?,
                    source_branch: row.get("cmt_source_branch")?,
                    target_branch: row.get("cmt_target_branch")?,
                },
            })
        })
        .log_error()?;

    let vec: Result<Vec<ChainmrStepResult>, _> = steps.collect();
    vec.log_error()
}

fn fail_task(
    chainmr_step: &ChainmrStepResult,
    conn: &Connection,
) -> Result<(), String> {
    conn.execute(
        "
            update chainmr__step set status = 'failed'
                where (id = ?1 or status = 'created') and task_id = ?2;
            update chainmr__task set status = 'failed'
                where id = ?2;
            ",
        params![chainmr_step.id, chainmr_step.task.id],
    )
    .log_error()?;

    Ok(())
}

fn on_close_mr(
    chainmr_step: &ChainmrStepResult,
    conn: &Connection,
) -> Result<(), String> {
    Notifier::notify(
        chainmr_step.get_title(),
        Some("Ошибка ChainMR: Текущий MR закрылся".to_string()),
        vec![(
            chainmr_step.mr_web_url.as_ref().ok_or("Нет ссылки на ME")?.clone(),
            "Открыть MR".to_string(),
        )],
        |url| {
            let _ = open::that(url).log_error();
        },
    );

    fail_task(chainmr_step, conn)?;

    Ok(())
}

fn on_merge_mr(
    chainmr_step: &ChainmrStepResult,
    conn: &Connection,
) -> Result<(), String> {
    if chainmr_step.step_number == chainmr_step.steps_count {
        let _ = conn.execute("\
        update chainmr__task
        set status = case when watch_pipline_after_complete is true then 'wait_pipeline' else 'success' end
        where id = ?1;
        update chainmr__step set status = 'success'
        where id = ?2;
        ", params![chainmr_step.task.id, chainmr_step.id]).log_error()?;
    } else {
        let _ = conn
            .execute(
                "\
        update chainmr__step
            set status = 'success'
            where id = ?1;
        update chainmr__step
            set status = 'pending'
            where task_id=?2 and step_number=?3;
                ",
                params![
                    chainmr_step.id,
                    chainmr_step.task.id,
                    chainmr_step.step_number + 1,
                ],
            )
            .log_error()?;
    };

    Ok(())
}

fn create_watch_mr(
    mr: &GlMergeRequest,
    chainmr_step: &ChainmrStepResult,
    conn: &Connection,
) -> Result<(), String> {
    conn.execute(
        "\
insert into watch__mr (mr_id, project_id, web_url, status, has_conflicts, notify_on_end,
                       auto_merge)
values (?1, ?2, ?3, 'opened', false, false, true);

    ", params![mr.iid.to_string(), chainmr_step.task.project_id, mr.web_url.to_string()],
    ).log_error()?;

    Ok(())
}

fn start_task(
    chainmr_step: &ChainmrStepResult,
    conn: &Connection,
    gitlab_manager: &GitlabManager,
) -> Result<(), String> {
    let mr_res = gitlab_manager.create_mr(
        chainmr_step.source_branch.to_string(),
        chainmr_step.target_branch.to_string(),
        chainmr_step.task.project_id as u64,
        Some(chainmr_step.target_branch.to_string()),
        match chainmr_step.source_branch.get_task_or_empty() {
            Some(task_iid) => Some(format!("#{}", task_iid)),
            None => None,
        },
    );

    match mr_res {
        Ok(mr) => {
            Notifier::notify(
                chainmr_step.get_title(),
                Some(format!(
                    "Создан MR {} ({} -> {})",
                    mr.iid, mr.source_branch, mr.target_branch
                )),
                vec![(mr.web_url.to_string(), "Открыть MR".to_string())],
                |url| {
                    let _ = open::that(url).log_error();
                },
            );

            create_watch_mr(&mr, chainmr_step, conn)?;
        }
        Err(err) => {
            let _reg = Regex::new(r"Another open merge request already exists for this source branch: (!\d+)").log_error()?;

            if let Some(exist_mr_match) = _reg.find(err.as_str()) {
                let exist_mr_id = exist_mr_match.as_str().replace("!", "");
                let exist_mr = gitlab_manager.get_merge_request(
                    chainmr_step.task.project_id as u64,
                    exist_mr_id.parse().unwrap(),
                )?;

                create_watch_mr(&exist_mr, chainmr_step, conn)?;

                Notifier::notify(
                    chainmr_step.get_title(),
                    Some(format!(
                        "Задача привязана к существующему MR !{} ({} -> {})",
                        exist_mr.iid,
                        exist_mr.source_branch,
                        exist_mr.target_branch
                    )),
                    vec![(
                        exist_mr.web_url.to_string(),
                        "Открыть MR".to_string(),
                    )],
                    |url| {
                        let _ = open::that(url).log_error();
                    },
                );

                return Ok(());
            }

            fail_task(chainmr_step, conn)?;
        }
    };

    Ok(())
}
