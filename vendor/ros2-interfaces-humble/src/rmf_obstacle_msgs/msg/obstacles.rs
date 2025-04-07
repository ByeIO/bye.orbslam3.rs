use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Obstacles {
    pub obstacles: Vec<crate::rmf_obstacle_msgs::msg::Obstacle>,
}

impl Default for Obstacles {
    fn default() -> Self {
        Obstacles {
            obstacles: Vec::new(),
        }
    }
}

impl ros2_client::Message for Obstacles {}
