use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CellularStatus {
    pub status: u8,
    pub failure_reason: u8,
    #[serde(rename = "type")]    pub type_: u8,
    pub quality: u8,
    pub mcc: u16,
    pub mnc: u16,
    pub lac: u16,
}

impl Default for CellularStatus {
    fn default() -> Self {
        CellularStatus {
            status: 0,
            failure_reason: 0,
            type_: 0,
            quality: 0,
            mcc: 0,
            mnc: 0,
            lac: 0,
        }
    }
}

impl ros2_client::Message for CellularStatus {}
