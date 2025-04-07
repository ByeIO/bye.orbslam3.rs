use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RobotArray {
    pub header: crate::std_msgs::msg::Header,
    pub robots: Vec<crate::soccer_vision_2d_msgs::msg::Robot>,
}

impl Default for RobotArray {
    fn default() -> Self {
        RobotArray {
            header: crate::std_msgs::msg::Header::default(),
            robots: Vec::new(),
        }
    }
}

impl ros2_client::Message for RobotArray {}
