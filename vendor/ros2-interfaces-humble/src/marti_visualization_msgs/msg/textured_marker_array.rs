use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TexturedMarkerArray {
    pub markers: Vec<crate::marti_visualization_msgs::msg::TexturedMarker>,
}

impl Default for TexturedMarkerArray {
    fn default() -> Self {
        TexturedMarkerArray {
            markers: Vec::new(),
        }
    }
}

impl ros2_client::Message for TexturedMarkerArray {}
