use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FloatStamped {
    pub header: crate::std_msgs::msg::Header,
    pub data: f64,
}

impl Default for FloatStamped {
    fn default() -> Self {
        FloatStamped {
            header: crate::std_msgs::msg::Header::default(),
            data: 0.0,
        }
    }
}

impl ros2_client::Message for FloatStamped {}
