use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IrIntensityVector {
    pub header: crate::std_msgs::msg::Header,
    pub readings: Vec<crate::irobot_create_msgs::msg::IrIntensity>,
}

impl Default for IrIntensityVector {
    fn default() -> Self {
        IrIntensityVector {
            header: crate::std_msgs::msg::Header::default(),
            readings: Vec::new(),
        }
    }
}

impl ros2_client::Message for IrIntensityVector {}
