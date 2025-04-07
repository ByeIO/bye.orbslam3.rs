use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Altitude {
    pub header: crate::std_msgs::msg::Header,
    pub altitude: f64,
    pub sigma: f64,
}

impl Default for Altitude {
    fn default() -> Self {
        Altitude {
            header: crate::std_msgs::msg::Header::default(),
            altitude: 0.0,
            sigma: 0.0,
        }
    }
}

impl ros2_client::Message for Altitude {}
