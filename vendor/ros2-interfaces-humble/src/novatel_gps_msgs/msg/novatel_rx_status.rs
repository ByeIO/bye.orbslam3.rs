use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NovatelRxStatus {
    pub header: crate::std_msgs::msg::Header,
    pub novatel_msg_header: crate::novatel_gps_msgs::msg::NovatelMessageHeader,
    pub error: u32,
    pub rxstat: u32,
    pub aux1stat: u32,
    pub aux2stat: u32,
    pub aux3stat: u32,
    pub aux4stat: u32,
}

impl Default for NovatelRxStatus {
    fn default() -> Self {
        NovatelRxStatus {
            header: crate::std_msgs::msg::Header::default(),
            novatel_msg_header: crate::novatel_gps_msgs::msg::NovatelMessageHeader::default(),
            error: 0,
            rxstat: 0,
            aux1stat: 0,
            aux2stat: 0,
            aux3stat: 0,
            aux4stat: 0,
        }
    }
}

impl ros2_client::Message for NovatelRxStatus {}
