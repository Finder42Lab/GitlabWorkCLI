use std::fmt::Display;

#[derive(Debug)]
pub enum ChainmrStatus {
    Created,
    Pending,
    Success,
    Failed,
    WaitPipeline,
}

impl From<String> for ChainmrStatus {
    fn from(s: String) -> Self {
        match s.as_ref() {
            "created" => ChainmrStatus::Created,
            "pending" => ChainmrStatus::Pending,
            "success" => ChainmrStatus::Success,
            "failed" => ChainmrStatus::Failed,
            "wait_pipeline" => ChainmrStatus::WaitPipeline,
            _ => ChainmrStatus::Failed,
        }
    }
}

impl Display for ChainmrStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            ChainmrStatus::Created => "created",
            ChainmrStatus::Pending => "pending",
            ChainmrStatus::Success => "success",
            ChainmrStatus::Failed => "failed",
            ChainmrStatus::WaitPipeline => "wait_pipeline",
        };
        write!(f, "{}", str)
    }
}
