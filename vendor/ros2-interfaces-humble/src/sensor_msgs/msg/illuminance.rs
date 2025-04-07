use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Illuminance {
    pub header: crate::std_msgs::msg::Header,
    pub illuminance: f64,
    pub variance: f64,
}

impl Default for Illuminance {
    fn default() -> Self {
        Illuminance {
            header: crate::std_msgs::msg::Header::default(),
            illuminance: 0.0,
            variance: 0.0,
        }
    }
}

impl ros2_client::Message for Illuminance {}
