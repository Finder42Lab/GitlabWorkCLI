use log::error;
use url::Url;
use helpers::{Printer, ProjectConfig};
use crate::structs::AppState;

pub fn init_command(app_state: &AppState) -> Result<(), String> {
    // Получаем ссылку на репозиторий
    let raw_remote = app_state.git_manager.get_repo_url()?;

    // Парсим ссылку в объект
    let remote = match Url::parse(raw_remote.as_str()) {
        Ok(url) => url,
        Err(err) => {
            error!("{:?}", err);
            return Err(err.to_string());
        }
    };

    // Сравниваем хосты в ссылке репозитория и конфиге утилиты
    if remote.host_str().unwrap() != app_state.app_config.gitlab_host {
        return Err(format!("Хост репозитория проекта отличен от {}", app_state.app_config.gitlab_host));
    }

    Printer::print("Получаю информацию о проекте...".to_string(), None);

    // Получаем токены пути
    let project_tokens = match remote.path_segments() {
        None => {
            return Err("Пустой адресс репозитория".to_string());
        }
        Some(t) => { t.collect::<Vec<&str>>() }
    };

    let group = app_state.gitlab_manager.get_group(project_tokens.first().unwrap().to_string())?;
    let project = app_state.gitlab_manager.get_project(project_tokens.join("/"))?;

    println!("Группа: {} ({})", group.name, group.web_url);
    println!("Проект: {} ({})", project.name, project.web_url);

    let project_config = ProjectConfig {
        project_id: project.id,
        group_id: group.id,
    };

    println!("Сохраняю конфиг...");

    match project_config.save(&app_state.path) {
        Ok(_) => {
            Printer::print_success("Проект успешно инициализирован!".to_string(), None);
            Ok(())
        }
        Err(err) => {
            Err(format!("Ошибка при сохрании конфигурации проекта: \n {}", err))
        }
    }


}