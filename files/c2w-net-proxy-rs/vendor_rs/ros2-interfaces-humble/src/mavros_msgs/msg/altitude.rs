use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Altitude {
    pub header: crate::std_msgs::msg::Header,
    pub monotonic: f32,
    pub amsl: f32,
    pub local: f32,
    pub relative: f32,
    pub terrain: f32,
    pub bottom_clearance: f32,
}

impl Default for Altitude {
    fn default() -> Self {
        Altitude {
            header: crate::std_msgs::msg::Header::default(),
            monotonic: 0.0,
            amsl: 0.0,
            local: 0.0,
            relative: 0.0,
            terrain: 0.0,
            bottom_clearance: 0.0,
        }
    }
}

impl ros2_client::Message for Altitude {}
