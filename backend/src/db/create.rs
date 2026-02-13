use helpers::LogError;
use rusqlite::Connection;
use std::path::PathBuf;

pub fn create_db(db_path: &PathBuf) -> Result<(), String> {
    let conn = Connection::open(db_path).log_error()?;

    conn.execute(
        "create table if not exists watch__piplines (
            id integer primary key,
            gl_pipline_id integer not null,
            project_id integer not null,
            web_url text not null,
            status varchar(20),
            fail_message text,
            sha text,
            notify_on_end bool default false
        )",
        (),
    )
    .log_error()?;

    conn.execute(
        "create table if not exists watch__mr (
            id integer primary key,
            mr_id integer not null,
            project_id integer not null,
            web_url text not null,
            status varchar(20),
            has_conflicts boolean default false,
            fail_message text,
            notify_on_end bool default false,
            merge_commit_sha text,
            auto_merge bool default false,
            watch_pipline_after_merge bool default false
        )",
        (),
    )
    .log_error()?;

    conn.execute(
        "create table if not exists chainmr__task (
            id integer primary key,
            status varchar(20) not null,
            source_branch varchar(40) not null,
            target_branch varchar(40) not null,
            fail_message text,
            watch_pipline_after_complete bool default false,
            watch_pipline_id integer
        )",
        (),
    )
    .log_error()?;

    conn.execute(
        "create table if not exists chainmr__step (
            id integer primary key,
            task_id integer references chainmr__task (id) not null,
            step_number integer not null default 0,
            status varchar(20) not null,
            source_branch varchar(40) not null,
            target_branch varchar(40) not null,
            watch_mr_id integer references watch__mr (id),
            fail_message text
        )",
        (),
    )
    .log_error()?;

    Ok(())
}
