use helpers::LogError;
use managers::GitlabManager;
use managers::gitlab::structs::GlMergeRequestState;
use rusqlite::Connection;
use std::path::PathBuf;

struct ChainmrStepResult {
    id: i32,
    task_id: i32,
    project_id: i32,
    step_number: i32,
    source_branch: String,
    target_branch: String,
    steps_count: i32,
    mr_status: GlMergeRequestState,
}

pub fn watch_chainmr(
    db_path: &PathBuf,
    gitlab_manager: &GitlabManager,
) -> Result<(), String> {
    let conn = Connection::open(db_path).log_error()?;

    let steps = load_steps(&conn)?;

    for step in steps {}

    Ok(())
}

fn load_steps(conn: &Connection) -> Result<Vec<ChainmrStepResult>, String> {
    let mut steps_query = conn
        .prepare(
            "\
        select cms.id,
               cms.task_id,
               cmt.project_id,
               cms.step_number,
               cms.source_branch,
               cms.target_branch,
               cmt.steps_count,
               wm.status
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
            Ok(ChainmrStepResult {
                id: row.get(0).log_error().unwrap(),
                task_id: row.get(1).log_error().unwrap(),
                project_id: row.get(2).log_error().unwrap(),
                step_number: row.get(3).log_error().unwrap(),
                source_branch: row.get(4).log_error().unwrap(),
                target_branch: row.get(5).log_error().unwrap(),
                steps_count: row.get(6).log_error().unwrap(),
                mr_status: GlMergeRequestState::from(
                    row.get::<_, String>(7).log_error().unwrap(),
                ),
            })
        })
        .log_error()?;

    Ok(steps.map(|i| i.log_error().unwrap()).collect())
}
