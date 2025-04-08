use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnvSensor {
    pub header: crate::std_msgs::msg::Header,
    #[serde(rename = "type")]    pub type_: i32,
    pub value: f64,
}

impl Default for EnvSensor {
    fn default() -> Self {
        EnvSensor {
            header: crate::std_msgs::msg::Header::default(),
            type_: 0,
            value: 0.0,
        }
    }
}

impl ros2_client::Message for EnvSensor {}
