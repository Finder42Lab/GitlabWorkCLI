use std::fs::File;
use std::path::PathBuf;
use clap::Parser;
use log::{error, LevelFilter};
use simplelog::{CombinedLogger, Config, WriteLogger};
use crate::cli::{Commands, GlobalConfigCommands, CLI};
use crate::commands::checkout_feature::checkout_feature_command;
use crate::commands::checkout_task::checkout_task_command;
use crate::commands::create_mr::create_mr_command;
use crate::commands::init::init_command;
use crate::commands::update_host::update_host_command;
use crate::commands::update_token::update_token_command;
use crate::git::GitManager;
use crate::gitlab::GitlabManager;
use crate::helpers::app_config::load_app_config;
use crate::helpers::printer::Printer;
use crate::helpers::project_config::load_project_config;
use crate::structs::{AppConfig, AppState, ProjectConfig};

mod structs;
mod cli;
mod helpers;
mod commands;
mod gitlab;
mod git;

fn get_app_state(app_config: AppConfig, current_dir: PathBuf) -> Option<AppState> {
    let gitlab_manager = match GitlabManager::new((&app_config).gitlab_token.to_string(), (&app_config).gitlab_host.to_string()) {
        Ok(gm) => gm,
        Err(err) => {
            Printer::print_error(err, Some("GitLab".to_string()));
            return None
        }
    };

    let git_manager = match GitManager::new(&current_dir) {
        Ok(gm) => gm,
        Err(err) => {
            Printer::print_error(err, Some("Git".to_string()));
            return None
        }
    };

    Some(AppState {
        app_config,
        git_manager,
        gitlab_manager,
        path: current_dir,
    })
}


fn process_core_commands(parsed_command: &Commands, config: &AppConfig) -> Option<()> {
    // Команды, которым не нужны менеджеры
    let res = match parsed_command {
        Commands::GlobalConfig(gc_command) =>
            match gc_command {
                GlobalConfigCommands::SetToken { token } => update_token_command(config, token),
                GlobalConfigCommands::SetHost { host } => update_host_command(config, host),
            },
        _ => {
            return None
        }
    };

    match res {
        Ok(_) => {}
        Err(err) => {
            Printer::print_error(err, None);
        }
    };

    Some(())
}


fn process_base_commands(parsed_command: &Commands, config: &AppConfig, app_state: &AppState) -> Option<()> {
    // Команды, которым не нужен конфиг проекта
    let res = match parsed_command {
        Commands::Init => init_command(app_state),
        _ => {
            return None
        }
    };

    match res {
        Ok(_) => {}
        Err(err) => {
            Printer::print_error(err, None);
        }
    };

    Some(())
}

fn process_commands(parsed_command: &Commands, project_config: &ProjectConfig, app_state: &AppState) -> Option<()> {
    // Команды, которым нужны все менеджеры и конфиги
    let res = match parsed_command {
        Commands::CheckoutFeature { feature_iid: feature } => checkout_feature_command(app_state, project_config, *feature),
        Commands::CheckoutTask { task_iid: task } => checkout_task_command(app_state, project_config, *task),
        Commands::MergeRequest { source, target, review } => create_mr_command(app_state, project_config, source.to_owned(), target.to_owned()),
        _ => {
            return None
        },
    };

    match res {
        Ok(_) => {}
        Err(err) => {
            Printer::print_error(err, None);
        }
    };

    Some(())
}




fn main() {
    println!();
    CombinedLogger::init(
        vec![
            WriteLogger::new(LevelFilter::Info, Config::default(), File::create("aworkcli.log").unwrap()),
        ]
    ).unwrap();

    let config = load_app_config();
    let parsed = CLI::parse();

    let current_dir = match std::env::current_dir() {
        Ok(path) => path,
        Err(err) => {
            error!("{}", err);
            return;
        }
    };

    let parsed_command = match parsed.command {
        None => {
            Printer::print_error("Команда не найдена".to_string(), None);
            return;
        }
        Some(command) => command,
    };


    match process_core_commands(&parsed_command, &config) {
        Some(_) => {return;}
        None => {}
    }


    let app_state = match get_app_state(config.clone(), current_dir) {
        None => {return;}
        Some(state) => {state}
    };

    match process_base_commands(&parsed_command, &config, &app_state) {
        Some(_) => {return;}
        None => {}
    }

    let project_config = match load_project_config(app_state.path.to_path_buf()) {
        Ok(pc) => pc,
        Err(error) => {
            Printer::print_error(error, None);
            return;
        }
    };

    match process_commands(&parsed_command, &project_config, &app_state) {
        Some(_) => {return;}
        None => {}
    }

    println!();
}

