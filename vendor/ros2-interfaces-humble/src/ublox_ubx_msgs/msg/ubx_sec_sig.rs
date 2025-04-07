use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UBXSecSig {
    pub header: crate::std_msgs::msg::Header,
    pub version: u8,
    pub jam_det_enabled: u8,
    pub jamming_state: u8,
    pub spf_det_enabled: u8,
    pub spoofing_state: u8,
    pub jam_num_cent_freqs: u8,
    pub jam_state_cent_freqs: Vec<crate::ublox_ubx_msgs::msg::JamStateCentFreq>,
}

impl UBXSecSig {
    pub const JAM_UNKNOWN: u8 = 0;
    pub const JAM_NO_JAMMING: u8 = 1;
    pub const JAM_WARNING: u8 = 2;
    pub const JAM_CRITICAL: u8 = 3;
    pub const SPF_UNKNOWN: u8 = 0;
    pub const SPF_NO_SPOOFING: u8 = 1;
    pub const SPF_SPOOFING_INDICATED: u8 = 2;
    pub const SPF_SPOOFING_AFFIRMED: u8 = 3;
}

impl Default for UBXSecSig {
    fn default() -> Self {
        UBXSecSig {
            header: crate::std_msgs::msg::Header::default(),
            version: 0,
            jam_det_enabled: 0,
            jamming_state: 0,
            spf_det_enabled: 0,
            spoofing_state: 0,
            jam_num_cent_freqs: 0,
            jam_state_cent_freqs: Vec::new(),
        }
    }
}

impl ros2_client::Message for UBXSecSig {}
