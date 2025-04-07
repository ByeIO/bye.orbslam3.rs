use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageMarkerArray {
    pub markers: Vec<crate::visualization_msgs::msg::ImageMarker>,
}

impl Default for ImageMarkerArray {
    fn default() -> Self {
        ImageMarkerArray {
            markers: Vec::new(),
        }
    }
}

impl ros2_client::Message for ImageMarkerArray {}
