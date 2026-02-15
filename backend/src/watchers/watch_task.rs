use helpers::{LogError, Notifier};
use managers::gitlab::structs::GlMergeRequestState;
use managers::GitlabManager;
use rusqlite::Connection;
use std::path::PathBuf;

struct ChainmrTaskResult {
    id: i32,
    project_id: i32,
    source_branch: String,
    target_branch: String,
}

struct ChainmrStepResult {
    id: i32,
    task_id: i32,
    step_number: i32,
    source_branch: String,
    target_branch: String,
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
            Some(GlMergeRequestState::Merged) => {}
            None => {}
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
               cms.task_id,
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
        where cmt.status == 'pending'
          and cmt.status == 'pending'
    ",
        )
        .log_error()?;

    let steps = steps_query
        .query_map([], |row| {
            let status: Option<String> = row.get(6).log_error().unwrap();
            Ok(ChainmrStepResult {
                id: row.get(0).log_error().unwrap(),
                task_id: row.get(1).log_error().unwrap(),
                step_number: row.get(2).log_error().unwrap(),
                source_branch: row.get(3).log_error().unwrap(),
                target_branch: row.get(4).log_error().unwrap(),
                steps_count: row.get(5).log_error().unwrap(),
                mr_status: match status {
                    None => None,
                    Some(status) => Some(GlMergeRequestState::from(status)),
                },
                mr_web_url: row.get("web_url").log_error().unwrap(),
                mr_id: row.get("mr_id").log_error().unwrap(),
                task: ChainmrTaskResult {
                    id: row.get("cmt_id").log_error().unwrap(),
                    project_id: row.get("project_id").log_error().unwrap(),
                    source_branch: row
                        .get("cmt_source_branch")
                        .log_error()
                        .unwrap(),
                    target_branch: row
                        .get("cmt_target_branch")
                        .log_error()
                        .unwrap(),
                },
            })
        })
        .log_error()?;

    Ok(steps.map(|i| i.log_error().unwrap()).collect())
}

fn on_close_mr(
    chainmr_step: &ChainmrStepResult,
    conn: &Connection,
) -> Result<(), String> {
    Notifier::notify(
        chainmr_step.get_title(),
        Some("Ошибка ChainMR: Текущий MR закрылся".to_string()),
        vec![(
            chainmr_step.mr_web_url.clone().unwrap(),
            "Открыть MR".to_string(),
        )],
        |url| {
            let _ = open::that(url).log_error();
        },
    );

    conn.execute(
        "\
            update chainmr__step set status = 'failed'
                where (id = ?1 or status = 'created') and task_id = ?2;
            update chainmr__task set status = 'failed'
                where id = ?2;
",
        (chainmr_step.id, chainmr_step.task.id),
    )
    .log_error()?;

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
        where id = ?1;
        ", ()).log_error()?;
    };

    Ok(())
}
