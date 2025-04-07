use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RobotGoalsArray {
    pub header: crate::std_msgs::msg::Header,
    pub robots: Vec<crate::tuw_multi_robot_msgs::msg::RobotGoals>,
}

impl Default for RobotGoalsArray {
    fn default() -> Self {
        RobotGoalsArray {
            header: crate::std_msgs::msg::Header::default(),
            robots: Vec::new(),
        }
    }
}

impl ros2_client::Message for RobotGoalsArray {}
