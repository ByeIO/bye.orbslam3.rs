use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SrrTrack {
    pub header: crate::std_msgs::msg::Header,
    pub can_tx_detect_valid_level: u8,
    pub can_tx_detect_status: bool,
    pub can_tx_detect_range_rate: f32,
    pub can_tx_detect_range: f32,
    pub can_tx_detect_angle: f32,
    pub can_tx_detect_amplitude: f32,
}

impl SrrTrack {
    pub const CAN_TX_DETECT_VALID_LEVEL_SUSPECT_DETECTION: u8 = 0;
    pub const CAN_TX_DETECT_VALID_LEVEL_LEVEL_1: u8 = 1;
    pub const CAN_TX_DETECT_VALID_LEVEL_LEVEL_2: u8 = 2;
    pub const CAN_TX_DETECT_VALID_LEVEL_LEVEL_3: u8 = 3;
    pub const CAN_TX_DETECT_VALID_LEVEL_LEVEL_4: u8 = 4;
    pub const CAN_TX_DETECT_VALID_LEVEL_LEVEL_5: u8 = 5;
    pub const CAN_TX_DETECT_VALID_LEVEL_LEVEL_6: u8 = 6;
    pub const CAN_TX_DETECT_VALID_LEVEL_LEVEL_7: u8 = 7;
    pub const CAN_TX_DETECT_STATUS_NO_DATA: bool = false;
    pub const CAN_TX_DETECT_STATUS_VALID_DATA_PRESENT: bool = true;
}

impl Default for SrrTrack {
    fn default() -> Self {
        SrrTrack {
            header: crate::std_msgs::msg::Header::default(),
            can_tx_detect_valid_level: 0,
            can_tx_detect_status: false,
            can_tx_detect_range_rate: 0.0,
            can_tx_detect_range: 0.0,
            can_tx_detect_angle: 0.0,
            can_tx_detect_amplitude: 0.0,
        }
    }
}

impl ros2_client::Message for SrrTrack {}
