use helpers::LogError;
use managers::GitlabManager;
use managers::gitlab::structs::GlPipeline;
use rusqlite::Connection;
use std::path::PathBuf;

struct WatchMr {
    merge_commit_sha: String,
    project_id: i32,
}

pub fn watch_merged_mrs(
    db_path: &PathBuf,
    gitlab_manager: &GitlabManager,
) -> Result<(), String> {
    let conn = Connection::open(db_path).log_error()?;

    let mut mrs_query = conn.prepare(
        "\
    select merge_commit_sha, project_id
    from watch__mr wmr
    where wmr.watch_pipline_after_merge
      and wmr.merge_commit_sha
      and not exists(select 1 from watch__piplines wp where wp.sha = wmr.merge_commit_sha)
  ").log_error()?;

    let watch_mrs = mrs_query
        .query_map([], |row| {
            Ok(WatchMr {
                merge_commit_sha: row.get(0).log_error().unwrap(),
                project_id: row.get(1).log_error().unwrap(),
            })
        })
        .log_error()?;

    for watch_mr in watch_mrs {
        if let Ok(watch_mr) = watch_mr {
            let pipeline = match gitlab_manager.get_pipeline_by_sha(
                watch_mr.project_id as u64,
                watch_mr.merge_commit_sha,
            )? {
                Some(pipeline) => pipeline,
                None => continue,
            };

            let _ = conn.execute(
                "\
                insert into watch__piplines
                    (gl_pipline_id, project_id, web_url, status, sha, notify_on_end)
                    values (?1, ?2, ?3, ?4, ?5, true)
                ",
                (
                    pipeline.id.to_string(),
                    watch_mr.project_id.to_string(),
                    pipeline.web_url,
                    pipeline.status.to_string(),
                    pipeline.sha,
                ),
            ).log_error();
        }
    }

    Ok(())
}
