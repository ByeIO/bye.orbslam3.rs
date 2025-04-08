use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GoalStatus {
    pub goal_id: crate::actionlib_msgs::msg::GoalID,
    pub status: u8,
    pub text: ::std::string::String,
}

impl GoalStatus {
    pub const PENDING: u8 = 0;
    pub const ACTIVE: u8 = 1;
    pub const PREEMPTED: u8 = 2;
    pub const SUCCEEDED: u8 = 3;
    pub const ABORTED: u8 = 4;
    pub const REJECTED: u8 = 5;
    pub const PREEMPTING: u8 = 6;
    pub const RECALLING: u8 = 7;
    pub const RECALLED: u8 = 8;
    pub const LOST: u8 = 9;
}

impl Default for GoalStatus {
    fn default() -> Self {
        GoalStatus {
            goal_id: crate::actionlib_msgs::msg::GoalID::default(),
            status: 0,
            text: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GoalStatus {}
