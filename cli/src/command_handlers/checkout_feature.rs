use helpers::{Printer, ProjectConfig};
use crate::structs::{AppState};

pub fn checkout_feature_command(app_state: &AppState, project_config: &ProjectConfig, feature: u16) -> Result<(), String> {
    if app_state.git_manager.is_dirty()? {
        return Err("Репозиторий содержит несохраненные файлы!".to_string());
    };

    Printer::print_info("Получаю информацию о фиче".to_string(), None);

    let epic = app_state.gitlab_manager.get_parent_epic(feature, project_config.group_id)?;

    Printer::print_info(format!("Эпик: {} ({})", epic.title, epic.web_url), None);

    if epic.is_techdebt() {
        Printer::print_info("Эпик техдолговый. Переключаю на ветку master".to_string(), None);
        app_state.git_manager.checkout("master".to_string(), None)?;
    } else {
        Printer::print_info(format!("Переключаюсь на ветку {}", epic.get_branch_name()), None);
        app_state.git_manager.checkout(epic.get_branch_name(), Some("master".to_string()))?;
    }

    Ok(())
}