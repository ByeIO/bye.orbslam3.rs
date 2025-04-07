use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScannerInfo2204 {
    pub device_id: u8,
    pub scanner_type: u8,
    pub scan_number: u16,
    pub start_angle: f32,
    pub end_angle: f32,
    pub yaw_angle: f32,
    pub pitch_angle: f32,
    pub roll_angle: f32,
    pub offset_x: f32,
    pub offset_y: f32,
    pub offset_z: f32,
}

impl ScannerInfo2204 {
    pub const ALASCA_XT: u8 = 3;
    pub const LUX_ECU: u8 = 4;
    pub const LUX_PROTOTYPE: u8 = 5;
    pub const LUX: u8 = 6;
    pub const SCALA_B1: u8 = 96;
}

impl Default for ScannerInfo2204 {
    fn default() -> Self {
        ScannerInfo2204 {
            device_id: 0,
            scanner_type: 0,
            scan_number: 0,
            start_angle: 0.0,
            end_angle: 0.0,
            yaw_angle: 0.0,
            pitch_angle: 0.0,
            roll_angle: 0.0,
            offset_x: 0.0,
            offset_y: 0.0,
            offset_z: 0.0,
        }
    }
}

impl ros2_client::Message for ScannerInfo2204 {}
