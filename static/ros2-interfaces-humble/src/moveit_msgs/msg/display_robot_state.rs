use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DisplayRobotState {
    pub state: crate::moveit_msgs::msg::RobotState,
    pub highlight_links: Vec<crate::moveit_msgs::msg::ObjectColor>,
    pub hide: bool,
}

impl Default for DisplayRobotState {
    fn default() -> Self {
        DisplayRobotState {
            state: crate::moveit_msgs::msg::RobotState::default(),
            highlight_links: Vec::new(),
            hide: false,
        }
    }
}

impl ros2_client::Message for DisplayRobotState {}
