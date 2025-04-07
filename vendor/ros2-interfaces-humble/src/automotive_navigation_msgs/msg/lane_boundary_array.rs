use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LaneBoundaryArray {
    pub boundaries: Vec<crate::automotive_navigation_msgs::msg::LaneBoundary>,
}

impl Default for LaneBoundaryArray {
    fn default() -> Self {
        LaneBoundaryArray {
            boundaries: Vec::new(),
        }
    }
}

impl ros2_client::Message for LaneBoundaryArray {}
