use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Polygon {
    pub vertices: Vec<crate::rmf_traffic_msgs::msg::PolygonVertex>,
}

impl Default for Polygon {
    fn default() -> Self {
        Polygon {
            vertices: Vec::new(),
        }
    }
}

impl ros2_client::Message for Polygon {}
