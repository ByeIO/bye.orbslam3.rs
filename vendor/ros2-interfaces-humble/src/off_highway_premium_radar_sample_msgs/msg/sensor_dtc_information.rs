use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SensorDtcInformation {
    pub header: crate::std_msgs::msg::Header,
    pub lgp_version: u32,
    pub dtcs: [u32; 10],
}

impl Default for SensorDtcInformation {
    fn default() -> Self {
        SensorDtcInformation {
            header: crate::std_msgs::msg::Header::default(),
            lgp_version: 0,
            dtcs: [0; 10],
        }
    }
}

impl ros2_client::Message for SensorDtcInformation {}
