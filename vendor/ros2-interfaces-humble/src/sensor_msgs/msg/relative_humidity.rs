use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RelativeHumidity {
    pub header: crate::std_msgs::msg::Header,
    pub relative_humidity: f64,
    pub variance: f64,
}

impl Default for RelativeHumidity {
    fn default() -> Self {
        RelativeHumidity {
            header: crate::std_msgs::msg::Header::default(),
            relative_humidity: 0.0,
            variance: 0.0,
        }
    }
}

impl ros2_client::Message for RelativeHumidity {}
