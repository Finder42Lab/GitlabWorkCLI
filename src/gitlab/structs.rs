use serde::{Deserialize, Serialize};


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


#[derive(Deserialize, Serialize, Debug)]
pub struct GlPipeline {
    id: u64,
    iid: u64,
    status: String,
    web_url: String,
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
    pub merge_status: String,
    pub has_conflicts: bool,
    pub head_pipeline: Option<GlPipeline>,
}
