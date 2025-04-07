use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AIMPlusStatus {
    pub header: crate::std_msgs::msg::Header,
    pub tow: u32,
    pub wnc: u16,
    pub interference: u8,
    pub spoofing: u8,
    pub osnma_authenticating: bool,
    pub galileo_authentic: u8,
    pub galileo_spoofed: u8,
    pub gps_authentic: u8,
    pub gps_spoofed: u8,
}

impl AIMPlusStatus {
    pub const SPECTRUM_CLEAN: u8 = 0;
    pub const INTERFERENCE_MITIGATED: u8 = 1;
    pub const INTERFERENCE_PRESENT: u8 = 2;
    pub const NONE_DETECTED: u8 = 0;
    pub const SPOOFING_DETECTED_BY_OSNMA: u8 = 1;
    pub const SPOOFING_DETECTED_BY_AUTHENTCITY_TEST: u8 = 2;
    pub const SPOOFING_DETECTED_BY_OSNMA_AND_AUTHENTCITY_TEST: u8 = 3;
}

impl Default for AIMPlusStatus {
    fn default() -> Self {
        AIMPlusStatus {
            header: crate::std_msgs::msg::Header::default(),
            tow: 0,
            wnc: 0,
            interference: 0,
            spoofing: 0,
            osnma_authenticating: false,
            galileo_authentic: 0,
            galileo_spoofed: 0,
            gps_authentic: 0,
            gps_spoofed: 0,
        }
    }
}

impl ros2_client::Message for AIMPlusStatus {}
