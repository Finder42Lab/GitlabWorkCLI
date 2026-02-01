use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use log::{info, warn};
use crate::structs::AppConfig;

pub fn load_app_config() -> AppConfig {
    let default_config = AppConfig {
        gitlab_token: "".to_string(),
        gitlab_host: "gitlab.example.com".to_string(),
    };

    let config_path = match get_app_config_file_path() {
        Some(p) => p,
        None => return default_config,
    };

    if !config_path.exists() {
        if let Ok(mut file) = File::create(&config_path) {
            let _ = serde_json::to_writer_pretty(&mut file, &default_config);
            info!("Создал файл конфига {}", config_path.display());
        }
        return default_config;
    };

    match File::open(&config_path) {
        Ok(file) => {
            match serde_json::from_reader(file) {
                Ok(config) => config,
                Err(e) => {
                    warn!("Ошибка загрузки конфига. Использую конфиг по умолчанию: \n {}", e.to_string());
                    default_config
                }
            }
        }
        Err(e) => {
            warn!("Ошибка чтения файла конфига. Использую конфиг по умолчанию: \n {}", e.to_string());
            default_config
        }
    }
}


pub fn get_app_config_dir() -> Option<PathBuf> {
    let path = dirs::config_local_dir()?
        .join("awork");

    if !path.exists() {
        fs::create_dir(&path).ok()?;
    }

    Some(path)
}

pub fn get_app_config_file_path() -> Option<PathBuf> {
    match get_app_config_dir() {
        Some(p) => Some(p.join("server.json")),
        None => None,
    }
}

pub fn save_app_config(config: AppConfig) -> Result<(), String> {
    let config_path = match get_app_config_file_path() {
        None => {return Err("Ошибка открытия файла".to_string())}
        Some(path) => {path}
    };

    let file = match File::create(config_path) {
        Ok(f) => {f}
        Err(err) => {
            return Err(err.to_string());
        }
    };
    let mut writer = BufWriter::new(file);

    serde_json::to_writer(&mut writer, &config).map_err(|err| err.to_string())?;
    writer.flush().map_err(|err| err.to_string())?;

    Ok(())
}
