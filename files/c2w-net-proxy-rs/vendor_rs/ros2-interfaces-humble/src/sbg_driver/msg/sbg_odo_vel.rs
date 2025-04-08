use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SbgOdoVel {
    pub header: crate::std_msgs::msg::Header,
    pub time_stamp: u32,
    pub status: bool,
    pub vel: f32,
}

impl Default for SbgOdoVel {
    fn default() -> Self {
        SbgOdoVel {
            header: crate::std_msgs::msg::Header::default(),
            time_stamp: 0,
            status: false,
            vel: 0.0,
        }
    }
}

impl ros2_client::Message for SbgOdoVel {}
