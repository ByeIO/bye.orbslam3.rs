use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScannerInfo2205 {
    pub device_id: u8,
    pub scanner_type: u8,
    pub scan_number: u16,
    pub start_angle: f32,
    pub end_angle: f32,
    pub scan_start_time: crate::builtin_interfaces::msg::Time,
    pub scan_end_time: crate::builtin_interfaces::msg::Time,
    pub scan_start_time_from_device: crate::builtin_interfaces::msg::Time,
    pub scan_end_time_from_device: crate::builtin_interfaces::msg::Time,
    pub scan_frequency: f32,
    pub beam_tilt: f32,
    pub scan_flags: u32,
    pub mounting_position: crate::ibeo_msgs::msg::MountingPositionF,
    pub resolutions: [crate::ibeo_msgs::msg::ResolutionInfo; 8],
}

impl ScannerInfo2205 {
    pub const ALASCA_XT: u8 = 3;
    pub const LUX_ECU: u8 = 4;
    pub const LUX_PROTOTYPE: u8 = 5;
    pub const LUX: u8 = 6;
    pub const SCALA_B1: u8 = 96;
}

impl Default for ScannerInfo2205 {
    fn default() -> Self {
        ScannerInfo2205 {
            device_id: 0,
            scanner_type: 0,
            scan_number: 0,
            start_angle: 0.0,
            end_angle: 0.0,
            scan_start_time: crate::builtin_interfaces::msg::Time::default(),
            scan_end_time: crate::builtin_interfaces::msg::Time::default(),
            scan_start_time_from_device: crate::builtin_interfaces::msg::Time::default(),
            scan_end_time_from_device: crate::builtin_interfaces::msg::Time::default(),
            scan_frequency: 0.0,
            beam_tilt: 0.0,
            scan_flags: 0,
            mounting_position: crate::ibeo_msgs::msg::MountingPositionF::default(),
            resolutions: core::array::from_fn(|_| crate::ibeo_msgs::msg::ResolutionInfo::default()),
        }
    }
}

impl ros2_client::Message for ScannerInfo2205 {}
