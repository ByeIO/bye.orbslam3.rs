use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Polygon {
    pub points: Vec<crate::geometry_msgs::msg::Point32>,
}

impl Default for Polygon {
    fn default() -> Self {
        Polygon {
            points: Vec::new(),
        }
    }
}

impl ros2_client::Message for Polygon {}
