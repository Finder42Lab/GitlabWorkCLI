mod db;
mod watchers;

use crate::db::create::create_db;
use crate::watchers::watch_merged_pipeline::watch_merged_mrs;
use crate::watchers::watch_mr::watch_mrs;
use crate::watchers::watch_pipeline::watch_pipelines;
use helpers::{LogError, get_app_config_dir, load_app_config};
use log::{LevelFilter, error};
use managers::GitlabManager;
use rouille::Response;
use simplelog::{CombinedLogger, Config, WriteLogger};
use std::fs::File;
use std::thread::{sleep, spawn};
use std::time::Duration;

fn main() {
    let dir = get_app_config_dir().unwrap();
    let log_path = dir.join("server.log");
    let db_path = dir.join("db.sqlite");

    CombinedLogger::init(vec![WriteLogger::new(
        LevelFilter::Info,
        Config::default(),
        File::create(log_path).unwrap(),
    )])
    .unwrap();

    match create_db(&db_path) {
        Err(err) => {
            error!("Не удалось создать/обновить БД: {}", err);
            return;
        }
        _ => {}
    };

    let app_config = load_app_config();
    let server_port = app_config.server_port;

    spawn(move || {
        let gitlab_manager = match GitlabManager::new(
            app_config.gitlab_token,
            app_config.gitlab_host,
        )
        .log_error()
        {
            Err(_) => {
                return;
            }
            Ok(manager) => manager,
        };

        loop {
            let _ = watch_merged_mrs(&db_path, &gitlab_manager);
            let _ = watch_pipelines(&db_path, &gitlab_manager);
            let _ = watch_mrs(&db_path, &gitlab_manager);

            sleep(Duration::from_secs(10));
        }
    });

    rouille::start_server(
        format!("127.0.0.1:{}", server_port),
        move |request| Response::text("hello world"),
    );
}
