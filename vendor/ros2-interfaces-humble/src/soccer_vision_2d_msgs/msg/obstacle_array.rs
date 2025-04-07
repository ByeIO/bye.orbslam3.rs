use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObstacleArray {
    pub header: crate::std_msgs::msg::Header,
    pub obstacles: Vec<crate::soccer_vision_2d_msgs::msg::Obstacle>,
}

impl Default for ObstacleArray {
    fn default() -> Self {
        ObstacleArray {
            header: crate::std_msgs::msg::Header::default(),
            obstacles: Vec::new(),
        }
    }
}

impl ros2_client::Message for ObstacleArray {}
