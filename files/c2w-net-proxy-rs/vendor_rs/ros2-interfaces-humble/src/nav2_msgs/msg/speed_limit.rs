use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpeedLimit {
    pub header: crate::std_msgs::msg::Header,
    pub percentage: bool,
    pub speed_limit: f64,
}

impl Default for SpeedLimit {
    fn default() -> Self {
        SpeedLimit {
            header: crate::std_msgs::msg::Header::default(),
            percentage: false,
            speed_limit: 0.0,
        }
    }
}

impl ros2_client::Message for SpeedLimit {}
