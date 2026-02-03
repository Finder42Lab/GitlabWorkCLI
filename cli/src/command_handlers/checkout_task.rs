use crate::command_handlers::checkout_feature_command;
use helpers::{Printer, ProjectConfig};
use crate::structs::AppState;

pub fn checkout_task_command(app_state: &AppState, project_config: &ProjectConfig, task_id: u64) -> Result<(), String> {
    if app_state.git_manager.is_dirty()? {
        return Err("Репозиторий содержит несохраненные файлы!".to_string());
    };

    let issue_branch = format!("{}-task", task_id);

    match app_state.git_manager.get_existed_branch(issue_branch.to_string()) {
        Ok(branch) => {
            Printer::print_info(format!("Ветка {} уже существует, переключаюсь", branch), None);
            app_state.git_manager.raw_checkout(branch, false)?;
            return Ok(());
        }
        Err(_) => {}
    }

    Printer::print_info("Получаю информацию о задаче!".to_string(), None);

    let issue = app_state.gitlab_manager.get_issue(task_id, project_config.project_id)?;

    Printer::print_info(format!("Задача: {} ({})", issue.title, issue.web_url), None);

    match issue.epic {
        None => {
            Printer::print_warning("У задачи не указан эпик. Начинаю от мастера".to_string(), None);
            app_state.git_manager.checkout(issue_branch, Some("master".to_string()))?;
        }
        Some(epic) => {
            checkout_feature_command(
                &app_state,
                &project_config,
                epic.iid,
            )?;

            app_state.git_manager.raw_checkout(issue_branch, true)?;
        }
    }

    Ok(())
}