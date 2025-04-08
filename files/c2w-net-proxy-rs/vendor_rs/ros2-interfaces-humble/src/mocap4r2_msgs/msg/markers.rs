use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Markers {
    pub header: crate::std_msgs::msg::Header,
    pub frame_number: u32,
    pub markers: Vec<crate::mocap4r2_msgs::msg::Marker>,
}

impl Default for Markers {
    fn default() -> Self {
        Markers {
            header: crate::std_msgs::msg::Header::default(),
            frame_number: 0,
            markers: Vec::new(),
        }
    }
}

impl ros2_client::Message for Markers {}
