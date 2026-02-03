use std::io::{BufRead};
use std::path::PathBuf;
use std::process::Command;
use git2::{BranchType, Repository};
use log::error;
use helpers::Printer;

pub struct GitManager {
    repository: Repository,
    dir: PathBuf,
}

impl GitManager {
    pub fn new(path: &PathBuf) -> Result<Self, String> {
        let repo = Repository::open(path).map_err(|err| err.to_string())?;

        Ok(GitManager { repository: repo, dir: path.to_path_buf() })
    }

    pub fn get_repo_url(&self) -> Result<String, String> {
        let remote = self.repository.find_remote("origin").map_err(|err| err.to_string())?;
        match remote.url() {
            None => {
                return Err("Не удалось найти ссылку на репозиторий".to_string())
            }
            Some(url) => Ok(url.to_string())
        }
    }

    pub fn pull(&self) -> Result<(), String> {
        match Command::new("git").current_dir(&self.dir).arg("pull").status() {
            Ok(_) => Ok(()),
            Err(err) => {
                error!("{:?}", err);
                Err(err.to_string())
            }
        }
    }

    pub fn get_current_branch(&self) -> Result<String, String> {
        Ok(self.repository.head().unwrap().name().unwrap().to_string().split("/").last().unwrap().to_string())
    }

    pub fn checkout(&self, target_branch: String, parent_branch: Option<String>) -> Result<(), String> {
        self.pull()?;
        match self.get_existed_branch(target_branch.to_string()) {
            Ok(branch) => {
                self.raw_checkout(branch, false)?;
                self.pull()?;
            }
            Err(err) => {
                match parent_branch {
                    Some(branch) => {
                        Printer::print_warning(format!("Ветка {} не найдена. Переключаюсь на родительскую ({}) и создаю целевую ветку...", target_branch, branch), Some("Git".to_string()));

                        let _parent = self.get_existed_branch(branch.to_string())?;
                        self.raw_checkout(_parent, false)?;
                        self.pull()?;
                        self.raw_checkout(target_branch, true)?;
                    }
                    None => {
                        return Err(err);
                    }
                }
            }
        };

        Ok(())
    }

    pub fn raw_checkout(&self, branch: String, create: bool) -> Result<(), String> {
        let mut command = Command::new("git");
        command.current_dir(&self.dir).arg("checkout");

        if create {
            command.arg("-b");
        }
        command.arg(branch);

        match command.status() {
            Ok(_) => Ok(()),
            Err(err) => {
                error!("{:?}", err);
                Err(err.to_string())
            }
        }
    }


    pub fn is_dirty(&self) -> Result<bool, String> {
        match Command::new("git").current_dir(&self.dir).arg("status").arg("-s").output() {
            Ok(output) => {
                if output.stdout.is_empty() {
                    return Ok(false);
                };

                let lines = output.stdout.lines().find(|l| !l.as_ref().unwrap().starts_with("??"));

                match lines {
                    None => Ok(false),
                    Some(Ok(_)) => Ok(true),
                    Some(Err(_)) => Ok(false)
                }
            }
            Err(err) => {
                error!("{:?}", err);
                Err(err.to_string())
            }
        }
    }

    pub fn get_existed_branch(&self, branch: String) -> Result<String, String> {
        if self.repository.find_branch(branch.as_str(), BranchType::Local).is_ok() {
            return Ok(branch);
        };

        let remote_branch = format!("origin/{}", branch);

        if self.repository.find_branch(&remote_branch, BranchType::Remote).is_ok() {
            return Ok(remote_branch);
        }

        Err(format!("Ну удалось найти ветку {} (или {})", branch, remote_branch))
    }
}