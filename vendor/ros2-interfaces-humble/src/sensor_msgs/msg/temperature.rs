use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Temperature {
    pub header: crate::std_msgs::msg::Header,
    pub temperature: f64,
    pub variance: f64,
}

impl Default for Temperature {
    fn default() -> Self {
        Temperature {
            header: crate::std_msgs::msg::Header::default(),
            temperature: 0.0,
            variance: 0.0,
        }
    }
}

impl ros2_client::Message for Temperature {}
