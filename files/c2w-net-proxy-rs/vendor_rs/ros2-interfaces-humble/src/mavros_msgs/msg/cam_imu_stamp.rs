use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CamIMUStamp {
    pub frame_stamp: crate::builtin_interfaces::msg::Time,
    pub frame_seq_id: i32,
}

impl Default for CamIMUStamp {
    fn default() -> Self {
        CamIMUStamp {
            frame_stamp: crate::builtin_interfaces::msg::Time::default(),
            frame_seq_id: 0,
        }
    }
}

impl ros2_client::Message for CamIMUStamp {}
