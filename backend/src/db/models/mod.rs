use managers::gitlab::structs::GlPiplineStatus;

// watch__piplines
#[derive(Debug)]
pub struct WatchPipiline {
    pub id: i32,
    pub gl_pipline_id: i32,
    pub project_id: i32,
    pub status: GlPiplineStatus,
    pub fail_message: String,
    pub notify_on_end: bool,
}

#[derive(Debug)]
pub struct WatchPipilineShort {
    pub id: i32,
    pub gl_pipline_id: i32,
    pub project_id: i32,
    pub notify_on_end: bool,
}

#[derive(Debug)]
pub struct WatchMr {
    pub id: i32,
    pub mr_id: i32,
    pub watch_pipline_id: Option<i32>,
    pub project_id: i32,
    pub status: String,
    pub fail_message: String,
}

#[derive(Debug)]
pub enum ChainmrStatus {
    CREATED,
    PENDING,
    COMPLETED,
    FAILED,
}

#[derive(Debug)]
pub struct ChainmrTask {
    pub id: i32,
    pub status: ChainmrStatus,
    pub source_branch: String,
    pub target_branch: String,
    pub fail_message: String,
}

pub struct ChainmrStep {
    pub id: i32,
    pub task_id: i32,
    pub order: i32,
    pub status: ChainmrStatus,
    pub source_branch: String,
    pub target_branch: String,
    pub watch_mr_id: Option<i32>,
    pub fail_message: String,
}
