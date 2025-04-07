use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Mouse {
    pub header: crate::std_msgs::msg::Header,
    pub integrated_x: f32,
    pub integrated_y: f32,
    pub frame_id: u32,
    pub last_squal: u8,
}

impl Default for Mouse {
    fn default() -> Self {
        Mouse {
            header: crate::std_msgs::msg::Header::default(),
            integrated_x: 0.0,
            integrated_y: 0.0,
            frame_id: 0,
            last_squal: 0,
        }
    }
}

impl ros2_client::Message for Mouse {}
