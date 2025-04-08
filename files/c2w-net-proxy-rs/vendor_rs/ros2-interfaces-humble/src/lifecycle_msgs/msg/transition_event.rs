use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransitionEvent {
    pub timestamp: u64,
    pub transition: crate::lifecycle_msgs::msg::Transition,
    pub start_state: crate::lifecycle_msgs::msg::State,
    pub goal_state: crate::lifecycle_msgs::msg::State,
}

impl Default for TransitionEvent {
    fn default() -> Self {
        TransitionEvent {
            timestamp: 0,
            transition: crate::lifecycle_msgs::msg::Transition::default(),
            start_state: crate::lifecycle_msgs::msg::State::default(),
            goal_state: crate::lifecycle_msgs::msg::State::default(),
        }
    }
}

impl ros2_client::Message for TransitionEvent {}
