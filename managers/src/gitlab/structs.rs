use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Deserialize, Serialize, Debug)]
pub struct GlUser {
    pub id: u64,
    pub username: String,
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GlEpic {
    pub iid: u16,
    pub title: String,
    pub parent_iid: Option<u16>,
    pub web_url: String,
    pub labels: Vec<String>,
}

impl GlEpic {
    pub fn is_techdebt(&self) -> bool {
        (self.labels.iter().find(|l| *l == "тип::техдолг").is_some()
            && self.labels.iter().find(|l| *l == "корневой эпик").is_none())
            || self.title.to_lowercase().contains("техдолг")
    }

    pub fn get_branch_name(&self) -> String {
        format!("feature/{}", self.iid)
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GlEpicShort {
    pub iid: u16,
    pub title: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GlIssue {
    pub iid: u64,
    pub title: String,
    pub web_url: String,
    pub epic: Option<GlEpicShort>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GlGroup {
    pub id: u64,
    pub name: String,
    pub web_url: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GlProject {
    pub id: u64,
    pub name: String,
    pub web_url: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum GlPipelineStatus {
    Created,
    WaitingForResource,
    Preparing,
    Pending,
    Running,
    Success,
    Failed,
    Canceled,
    Skipped,
    Manual,
    Scheduled,
}

impl From<String> for GlPipelineStatus {
    fn from(s: String) -> Self {
        match s.as_ref() {
            "created" => GlPipelineStatus::Created,
            "waiting_for_resource" => GlPipelineStatus::WaitingForResource,
            "preparing" => GlPipelineStatus::Preparing,
            "pending" => GlPipelineStatus::Pending,
            "running" => GlPipelineStatus::Running,
            "success" => GlPipelineStatus::Success,
            "failed" => GlPipelineStatus::Failed,
            "cancelled" => GlPipelineStatus::Canceled,
            "skipped" => GlPipelineStatus::Skipped,
            "manual" => GlPipelineStatus::Manual,
            "scheduled" => GlPipelineStatus::Scheduled,
            _ => GlPipelineStatus::Failed,
        }
    }
}

impl Display for GlPipelineStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            GlPipelineStatus::Created => "created",
            GlPipelineStatus::WaitingForResource => "waiting_for_resource",
            GlPipelineStatus::Preparing => "preparing",
            GlPipelineStatus::Pending => "pending",
            GlPipelineStatus::Running => "running",
            GlPipelineStatus::Success => "success",
            GlPipelineStatus::Failed => "failed",
            GlPipelineStatus::Canceled => "cancelled",
            GlPipelineStatus::Skipped => "skipped",
            GlPipelineStatus::Manual => "manual",
            GlPipelineStatus::Scheduled => "scheduled",
            _ => "failed",
        };
        write!(f, "{}", str)
    }
}

impl GlPipelineStatus {
    pub fn is_failed(&self) -> bool {
        match self {
            GlPipelineStatus::Failed
            | GlPipelineStatus::Canceled
            | GlPipelineStatus::Skipped => true,
            _ => false,
        }
    }
    pub fn is_success(&self) -> bool {
        let t = [(1, 2)];
        match self {
            GlPipelineStatus::Success => true,
            _ => false,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GlPipeline {
    pub id: u64,
    pub iid: u64,
    pub status: GlPipelineStatus,
    pub web_url: String,
    pub sha: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum GlMergeRequestState {
    Closed,
    Opened,
    Locked,
    Merged,
}

impl From<String> for GlMergeRequestState {
    fn from(s: String) -> Self {
        match s.as_ref() {
            "closed" => GlMergeRequestState::Closed,
            "opened" => GlMergeRequestState::Opened,
            "locked" => GlMergeRequestState::Locked,
            "merged" => GlMergeRequestState::Merged,
            _ => GlMergeRequestState::Closed,
        }
    }
}

impl Display for GlMergeRequestState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            GlMergeRequestState::Closed => "closed",
            GlMergeRequestState::Opened => "opened",
            GlMergeRequestState::Locked => "locked",
            GlMergeRequestState::Merged => "merged",
        };
        write!(f, "{}", str)
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GlMergeRequest {
    pub id: u64,
    pub iid: u64,
    pub title: String,
    pub web_url: String,
    pub assignees: Vec<GlUser>,
    pub reviewers: Vec<GlUser>,
    pub target_branch: String,
    pub source_branch: String,
    pub state: GlMergeRequestState,
    pub has_conflicts: bool,
    pub head_pipeline: Option<GlPipeline>,
    pub merge_commit_sha: Option<String>,
}
