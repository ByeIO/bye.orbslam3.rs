use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NormalizedPointOfInterest2DStamped {
    pub header: crate::std_msgs::msg::Header,
    pub x: f32,
    pub y: f32,
    pub c: f32,
}

impl Default for NormalizedPointOfInterest2DStamped {
    fn default() -> Self {
        NormalizedPointOfInterest2DStamped {
            header: crate::std_msgs::msg::Header::default(),
            x: 0.0,
            y: 0.0,
            c: 0.0,
        }
    }
}

impl ros2_client::Message for NormalizedPointOfInterest2DStamped {}
