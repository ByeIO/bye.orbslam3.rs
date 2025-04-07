use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DispatchStates {
    pub active: Vec<crate::rmf_task_msgs::msg::DispatchState>,
    pub finished: Vec<crate::rmf_task_msgs::msg::DispatchState>,
}

impl Default for DispatchStates {
    fn default() -> Self {
        DispatchStates {
            active: Vec::new(),
            finished: Vec::new(),
        }
    }
}

impl ros2_client::Message for DispatchStates {}
