use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{mist_task_state::MistTaskState, mist_task_type::MistTaskType};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MistTask {
    pub id: Uuid,
    pub r#type: MistTaskType,
    pub state: MistTaskState,
    pub description: String,
    pub download_size: Option<u64>,
}
