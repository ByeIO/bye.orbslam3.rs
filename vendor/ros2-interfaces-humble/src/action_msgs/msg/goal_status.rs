use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GoalStatus {
    pub goal_info: crate::action_msgs::msg::GoalInfo,
    pub status: i8,
}

impl GoalStatus {
    pub const STATUS_UNKNOWN: i8 = 0;
    pub const STATUS_ACCEPTED: i8 = 1;
    pub const STATUS_EXECUTING: i8 = 2;
    pub const STATUS_CANCELING: i8 = 3;
    pub const STATUS_SUCCEEDED: i8 = 4;
    pub const STATUS_CANCELED: i8 = 5;
    pub const STATUS_ABORTED: i8 = 6;
}

impl Default for GoalStatus {
    fn default() -> Self {
        GoalStatus {
            goal_info: crate::action_msgs::msg::GoalInfo::default(),
            status: 0,
        }
    }
}

impl ros2_client::Message for GoalStatus {}
