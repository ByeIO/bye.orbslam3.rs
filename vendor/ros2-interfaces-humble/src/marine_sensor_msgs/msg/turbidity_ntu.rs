use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TurbidityNTU {
    pub header: crate::std_msgs::msg::Header,
    pub ntu: f32,
}

impl Default for TurbidityNTU {
    fn default() -> Self {
        TurbidityNTU {
            header: crate::std_msgs::msg::Header::default(),
            ntu: 0.0,
        }
    }
}

impl ros2_client::Message for TurbidityNTU {}
