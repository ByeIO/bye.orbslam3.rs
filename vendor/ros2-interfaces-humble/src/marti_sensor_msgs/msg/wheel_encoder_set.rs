use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WheelEncoderSet {
    pub header: crate::std_msgs::msg::Header,
    pub encoders: Vec<crate::marti_sensor_msgs::msg::WheelEncoder>,
}

impl Default for WheelEncoderSet {
    fn default() -> Self {
        WheelEncoderSet {
            header: crate::std_msgs::msg::Header::default(),
            encoders: Vec::new(),
        }
    }
}

impl ros2_client::Message for WheelEncoderSet {}
