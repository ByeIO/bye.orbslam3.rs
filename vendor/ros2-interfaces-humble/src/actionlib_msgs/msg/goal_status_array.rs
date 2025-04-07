use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GoalStatusArray {
    pub header: crate::std_msgs::msg::Header,
    pub status_list: Vec<crate::actionlib_msgs::msg::GoalStatus>,
}

impl Default for GoalStatusArray {
    fn default() -> Self {
        GoalStatusArray {
            header: crate::std_msgs::msg::Header::default(),
            status_list: Vec::new(),
        }
    }
}

impl ros2_client::Message for GoalStatusArray {}
