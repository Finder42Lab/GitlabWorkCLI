use std::path::PathBuf;
use helpers::AppConfig;
use managers::git::GitManager;
use managers::gitlab::GitlabManager;

pub struct AppState {
    pub app_config: AppConfig,
    pub gitlab_manager: GitlabManager,
    pub git_manager: GitManager,
    pub path: PathBuf,
}

