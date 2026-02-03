use std::fmt::{Debug, Display};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use log::error;
use serde::{Deserialize, Serialize};
use crate::project_config::get_project_config_file_path;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AppConfig {
    pub gitlab_token: String,
    pub gitlab_host: String,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectConfig {
    pub project_id: u64,
    pub group_id: u64,
}

impl ProjectConfig {
    pub fn save(&self, dir: &PathBuf) -> Result<(), String> {
        let config_path = get_project_config_file_path(dir);

        let file = match File::create(config_path) {
            Ok(f) => {f}
            Err(err) => {
                error!("{:?}", err);
                return Err(err.to_string());
            }
        };
        let mut writer = BufWriter::new(file);

        serde_json::to_writer(&mut writer, &self).map_err(|err| err.to_string())?;
        writer.flush().map_err(|err| err.to_string())?;

        Ok(())
    }
}


pub trait LogError<T>  {
    fn log_error(self,) -> Result<T, String>;
}

impl<T, E> LogError<T> for Result<T, E>
where
    E: Display + Debug,
{
    fn log_error(self) -> Result<T, String> {
        if let Err(ref e) = self {
            error!("{:?}", e);
            return Err(e.to_string());
        }
        Ok(self.unwrap())
    }
}
