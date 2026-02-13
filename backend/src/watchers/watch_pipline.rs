use helpers::{AppConfig, LogError, Notifier};
use managers::GitlabManager;
use rusqlite::Connection;
use std::path::PathBuf;

pub struct WatchPipilineResult {
    pub id: i32,
    pub gl_pipline_id: i32,
    pub project_id: i32,
    pub notify_on_end: bool,
}

pub fn watch_piplines(
    db_path: &PathBuf,
    gitlab_manager: &GitlabManager,
) -> Result<(), String> {
    let conn = Connection::open(db_path).log_error()?;

    let mut piplines_query = conn
        .prepare(
            "\
    select wp.id, wp.gl_pipline_id, wp.project_id, wp.notify_on_end
    from watch__piplines wp
    where wp.status not in ('success', 'failed', 'canceled');
    ",
        )
        .log_error()?;

    let piplines = piplines_query
        .query_map([], |row| {
            Ok(WatchPipilineResult {
                id: row.get(0).unwrap(),
                gl_pipline_id: row.get(1).unwrap(),
                project_id: row.get(2).unwrap(),
                notify_on_end: row.get(3).unwrap(),
            })
        })
        .log_error()?;

    for watch_pipeline in piplines {
        if let Ok(watch_pipeline) = watch_pipeline {
            let pipeline = gitlab_manager.get_pipline(
                watch_pipeline.project_id as u64,
                watch_pipeline.gl_pipline_id as u64,
            )?;

            let res = conn
                .execute(
                    "update watch__piplines set status = ?1 where id = ?2",
                    (pipeline.status.to_string(), watch_pipeline.id),
                )
                .log_error();

            if res.is_err() | !watch_pipeline.notify_on_end {
                continue;
            }

            let mut title = "".to_string();
            let mut message = "".to_string();

            if pipeline.status.is_failed() {
                title = "Ошибка пайплайна".to_string();
                message =
                    format!("Пайплайн {} упал или был отменен", pipeline.id);
            }

            if pipeline.status.is_success() {
                title = "Пайплайн завершился!".to_string();
                message =
                    format!("Пайплайн {} успешно выполнился!", pipeline.id);
            }

            if title.is_empty() {
                continue;
            }

            Notifier::notify(
                title,
                Some(message),
                vec![(pipeline.web_url, "Открыть пайплайн".to_string())],
                |url| {
                    let _ = open::that(url).log_error();
                },
            );
        }
    }

    Ok(())
}
