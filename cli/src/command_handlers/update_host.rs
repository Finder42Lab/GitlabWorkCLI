use helpers::{save_app_config, AppConfig, Printer};

pub fn update_host_command(app_config: &AppConfig, host: &String) -> Result<(), String> {
    let mut new_config = app_config.clone();
    new_config.gitlab_host = host.to_string();

    match save_app_config(new_config) {
        Ok(_) => {
            Printer::print_success("Токен успешно обновлен!".to_string(), None);
        }
        Err(err) => {
            return Err(err)
        }
    };

    Ok(())

}