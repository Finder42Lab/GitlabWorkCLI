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
        (self
            .labels
            .iter()
            .find(|l| *l == "тип::техдолг")
            .is_some()
            && self
                .labels
                .iter()
                .find(|l| *l == "корневой эпик")
                .is_none())
            || self
                .title
                .to_lowercase()
                .contains("техдолг")
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

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum GlPiplineStatus {
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

impl From<String> for GlPiplineStatus {
    fn from(s: String) -> Self {
        match s.as_ref() {
            "created" => GlPiplineStatus::Created,
            "waiting_for_resource" => GlPiplineStatus::WaitingForResource,
            "preparing" => GlPiplineStatus::Preparing,
            "pending" => GlPiplineStatus::Pending,
            "running" => GlPiplineStatus::Running,
            "success" => GlPiplineStatus::Success,
            "failed" => GlPiplineStatus::Failed,
            "cancelled" => GlPiplineStatus::Canceled,
            "skipped" => GlPiplineStatus::Skipped,
            "manual" => GlPiplineStatus::Manual,
            "scheduled" => GlPiplineStatus::Scheduled,
            _ => GlPiplineStatus::Failed,
        }
    }
}

impl Display for GlPiplineStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            GlPiplineStatus::Created => "created",
            GlPiplineStatus::WaitingForResource => "waiting_for_resource",
            GlPiplineStatus::Preparing => "preparing",
            GlPiplineStatus::Pending => "pending",
            GlPiplineStatus::Running => "running",
            GlPiplineStatus::Success => "success",
            GlPiplineStatus::Failed => "failed",
            GlPiplineStatus::Canceled => "cancelled",
            GlPiplineStatus::Skipped => "skipped",
            GlPiplineStatus::Manual => "manual",
            GlPiplineStatus::Scheduled => "scheduled",
            _ => "failed",
        };
        write!(f, "{}", str)
    }
}

impl GlPiplineStatus {
    pub fn is_cancelled(&self) -> bool {
        match self {
            GlPiplineStatus::Failed
            | GlPiplineStatus::Canceled
            | GlPiplineStatus::Skipped => true,
            _ => false,
        }
    }
    pub fn is_success(&self) -> bool {
        let t = [(1, 2)];
        match self {
            GlPiplineStatus::Success => true,
            _ => false,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GlPipeline {
    pub id: u64,
    pub iid: u64,
    pub status: GlPiplineStatus,
    pub web_url: String,
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
