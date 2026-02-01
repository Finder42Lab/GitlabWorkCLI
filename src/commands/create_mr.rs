use crate::helpers::printer::Printer;
use crate::structs::{AppState, ProjectConfig};

pub fn create_mr_command(app_state: &AppState, project_config: &ProjectConfig, source_branch: Option<String>, target_branch: Option<String>) -> Result<(), String> {
    let source_branch = match source_branch {
        None => app_state.git_manager.get_current_branch()?,
        Some(b) => b
    };

    Printer::print_info(format!("Исходная ветка: {}", source_branch), None);
    Printer::print_info("Определяю целевую ветку".to_string(), None);

    let target_branch = match target_branch {
        Some(b) => b,
        None => {
            let target;
            if source_branch.ends_with("-task") {
                let task_iid: u64 = source_branch.replace("-task", "").parse().unwrap();

                let issue = app_state.gitlab_manager.get_issue(task_iid, project_config.project_id)?;

                if issue.epic.is_none() {
                    return Err("Не удалос определить цеевую ветку".to_string())
                }

                let epic = app_state.gitlab_manager.get_parent_epic(issue.epic.unwrap().iid, project_config.group_id)?;

                if epic.is_techdebt() {
                    target = "stage".to_string();
                } else {
                    target = epic.get_branch_name();
                }
            } else {
                return Err("Не удалось определить целевую ветку".to_string())
            }

            target
        },
    };

    Printer::print_info(format!("Целевая ветка: {}", target_branch), None);
    Printer::print_info("Создаю MR...".to_string(), None);

    let mr = app_state.gitlab_manager.create_mr(
        source_branch,
        target_branch,
        project_config.project_id,
    )?;

    Printer::print_success(format!("Создан MR !{} ({})", mr.iid, mr.web_url), None);

    Ok(())
}