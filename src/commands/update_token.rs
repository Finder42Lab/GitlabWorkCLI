use crate::helpers::app_config::save_app_config;
use crate::helpers::printer::Printer;
use crate::structs::{AppConfig};

pub fn update_token_command(app_config: &AppConfig, token: &String) -> Result<(), String> {
    let mut new_config = app_config.clone();
    new_config.gitlab_token = token.to_string();

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