use crate::structs::AppState;
use helpers::{Branch, Printer, ProjectConfig};
pub fn create_mr_command(
    app_state: &AppState,
    project_config: &ProjectConfig,
    source_branch: Option<Branch>,
    target_branch: Option<Branch>,
) -> Result<(), String> {
    let source_branch = match source_branch {
        None => app_state.git_manager.get_current_branch()?,
        Some(b) => b,
    };

    Printer::print_info(format!("Исходная ветка: {}", source_branch), None);
    Printer::print_info("Определяю целевую ветку".to_string(), None);

    let mr_title;

    let target_branch = match target_branch {
        Some(b) => {
            mr_title = b.to_string();
            b
        }
        None => {
            let target;

            match source_branch.get_task_or_empty() {
                None => {
                    return Err(
                        "Не удалось определить целевую ветку".to_string()
                    );
                }
                Some(task_iid) => {
                    let issue = app_state
                        .gitlab_manager
                        .get_issue(task_iid, project_config.project_id)?;

                    if issue.epic.is_none() {
                        return Err(
                            "Не удалос определить цеевую ветку".to_string()
                        );
                    }

                    mr_title = format!("Resolve: {}", issue.title);

                    let epic = app_state.gitlab_manager.get_parent_epic(
                        issue.epic.unwrap().iid,
                        project_config.group_id,
                    )?;

                    if epic.is_techdebt() {
                        target = Branch::new("stage".to_string());
                    } else {
                        target = epic.get_branch_name();
                    }
                }
            };

            target
        }
    };

    Printer::print_info(format!("Целевая ветка: {}", target_branch), None);
    Printer::print_info("Создаю MR...".to_string(), None);

    let description = match source_branch.get_task_or_empty() {
        None => "".to_string(),
        Some(task_iid) => {
            format!("#{}", task_iid)
        }
    };

    let mr = app_state.gitlab_manager.create_mr(
        source_branch.to_string(),
        target_branch.to_string(),
        project_config.project_id,
        Some(mr_title),
        Some(description),
    )?;

    Printer::print_success(
        format!("Создан MR !{} ({})", mr.iid, mr.web_url),
        None,
    );

    Ok(())
}
