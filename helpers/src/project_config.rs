use std::fs::File;
use std::path::PathBuf;
use log::error;
use crate::structs::ProjectConfig;

pub fn load_project_config(dir: PathBuf) -> Result<ProjectConfig, String> {
    let config_path = get_project_config_file_path(&dir);

    match File::open(&config_path) {
        Ok(file) => {
            match serde_json::from_reader(file) {
                Ok(config) => Ok(config),
                Err(e) => {
                    error!("{:?}", e);
                    Err("Не удалось прочитать конфиг проекта.".to_string())
                }
            }
        }
        Err(e) => {
            error!("{:?}", e);
            Err("Не удалось открыть конфиг проекта.\n Попробуйте выполнить команду init".to_string())
        }
    }
}


pub fn get_project_config_file_path(dir: &PathBuf) -> PathBuf {
    dir.join(".aworkcli")
}
