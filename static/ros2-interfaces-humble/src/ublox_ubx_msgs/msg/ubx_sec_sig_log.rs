use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UBXSecSigLog {
    pub header: crate::std_msgs::msg::Header,
    pub version: u8,
    pub num_events: u8,
    pub events: Vec<crate::ublox_ubx_msgs::msg::SigLogEvent>,
}

impl Default for UBXSecSigLog {
    fn default() -> Self {
        UBXSecSigLog {
            header: crate::std_msgs::msg::Header::default(),
            version: 0,
            num_events: 0,
            events: Vec::new(),
        }
    }
}

impl ros2_client::Message for UBXSecSigLog {}
