use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StateMachine {
    pub states: Vec<crate::yasmin_msgs::msg::State>,
}

impl Default for StateMachine {
    fn default() -> Self {
        StateMachine {
            states: Vec::new(),
        }
    }
}

impl ros2_client::Message for StateMachine {}
