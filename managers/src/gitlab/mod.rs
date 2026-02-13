mod builders;
pub mod structs;

use crate::gitlab::builders::EpicApi;
use crate::gitlab::structs::{
    GlEpic, GlGroup, GlIssue, GlMergeRequest, GlPipeline, GlProject, GlUser,
};
use gitlab::Gitlab;
use gitlab::api::{Query, groups, projects, users};
use helpers::LogError;
use log::error;

#[derive(Clone)]
pub struct GitlabManager {
    pub client: Gitlab,
}

// High level
impl GitlabManager {
    pub fn new(token: String, host: String) -> Result<Self, String> {
        if token.is_empty() {
            return Err("Не указан токен".to_string()); // TODO Добавить команду установки токена
        }

        let client = Gitlab::builder(host, token).cert_insecure().build();

        match client {
            Ok(cl) => Ok(Self { client: cl }),
            Err(err) => {
                error!("{:?}", err);
                Err(err.to_string())
            }
        }
    }

    pub fn get_issue(
        &self,
        task: u64,
        project_id: u64,
    ) -> Result<GlIssue, String> {
        let issue_url = match projects::issues::Issue::builder()
            .project(project_id)
            .issue(task)
            .build()
        {
            Ok(url) => url,
            Err(err) => {
                error!("{:?}", err);
                return Err(err.to_string());
            }
        };

        let issue: GlIssue = match issue_url.query(&self.client) {
            Ok(i) => i,
            Err(err) => {
                error!("{:?}", err);
                return Err(err.to_string());
            }
        };

        Ok(issue)
    }

    pub fn get_parent_epic(
        &self,
        epic_iid: u16,
        group_id: u64,
    ) -> Result<GlEpic, String> {
        let epic_url =
            match EpicApi::builder().group_id(group_id).iid(epic_iid).build() {
                Ok(url) => url,
                Err(err) => {
                    error!("{:?}", err);
                    return Err(err.to_string());
                }
            };

        let epic: GlEpic = match epic_url.query(&self.client) {
            Ok(epic) => epic,
            Err(err) => {
                error!("{:?}", err);
                return Err(err.to_string());
            }
        };

        match epic.parent_iid {
            Some(iid) => self.get_parent_epic(iid, group_id),
            None => Ok(epic),
        }
    }

    pub fn create_mr(
        &self,
        source_branch: String,
        target_branch: String,
        project_id: u64,
        title: Option<String>,
        description: Option<String>,
    ) -> Result<GlMergeRequest, String> {
        let current_user = self.get_current_user()?;

        let description = description.unwrap_or_else(|| "".to_string());
        let title = title.unwrap_or_else(|| target_branch.to_string());

        let request = projects::merge_requests::CreateMergeRequest::builder()
            .project(project_id)
            .source_branch(source_branch)
            .target_branch(target_branch.to_string())
            .title(title)
            .description(description)
            .assignee(current_user.id)
            .build()
            .log_error()?;

        let mr: GlMergeRequest = request.query(&self.client).log_error()?;

        Ok(mr)
    }
}

impl GitlabManager {
    pub fn get_group(&self, group: String) -> Result<GlGroup, String> {
        let group_url = groups::Group::builder()
            .group(group)
            .build()
            .map_err(|e| e.to_string())?;
        let group: GlGroup =
            group_url.query(&self.client).map_err(|e| e.to_string())?;

        Ok(group)
    }
    pub fn get_project(&self, project: String) -> Result<GlProject, String> {
        let project_url = projects::Project::builder()
            .project(project)
            .build()
            .map_err(|e| e.to_string())?;
        let project: GlProject =
            project_url.query(&self.client).map_err(|e| e.to_string())?;

        Ok(project)
    }

    pub fn get_current_user(&self) -> Result<GlUser, String> {
        let url = match users::CurrentUser::builder().build() {
            Ok(u) => u,
            Err(err) => {
                error!("{:?}", err);
                return Err(err.to_string());
            }
        };

        let user: GlUser = match url.query(&self.client) {
            Ok(u) => u,
            Err(err) => {
                error!("{:?}", err);
                return Err(err.to_string());
            }
        };

        Ok(user)
    }

    pub fn get_pipline(
        &self,
        project: u64,
        pipline_id: u64,
    ) -> Result<GlPipeline, String> {
        let url = projects::pipelines::Pipeline::builder()
            .project(project)
            .pipeline(pipline_id)
            .build()
            .log_error()?;
        let pipline: GlPipeline = url.query(&self.client).log_error()?;

        Ok(pipline)
    }

    pub fn get_pipline_by_sha(
        &self,
        project: u64,
        sha: String,
    ) -> Result<Option<GlPipeline>, String> {
        let url = projects::pipelines::Pipelines::builder()
            .project(project)
            .sha(sha)
            .build()
            .log_error()?;
        let pipline: Vec<GlPipeline> = url.query(&self.client).log_error()?;

        if pipline.is_empty() {
            return Ok(None);
        }

        Ok(Some(pipline[0].clone()))
    }

    pub fn get_merge_request(
        &self,
        project: u64,
        mr_id: u64,
    ) -> Result<GlMergeRequest, String> {
        let url = projects::merge_requests::MergeRequest::builder()
            .project(project)
            .merge_request(mr_id)
            .build()
            .log_error()?;

        let mr: GlMergeRequest = url.query(&self.client).log_error()?;

        Ok(mr)
    }

    pub fn merge_mr(
        &self,
        project: u64,
        mr_id: u64,
    ) -> Result<GlMergeRequest, String> {
        let url = projects::merge_requests::MergeMergeRequest::builder()
            .project(project)
            .merge_request(mr_id)
            .build()
            .log_error()?;

        let mr = url.query(&self.client).log_error()?;

        Ok(mr)
    }
}
