use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScanData2208 {
    pub header: crate::std_msgs::msg::Header,
    pub ibeo_header: crate::ibeo_msgs::msg::IbeoDataHeader,
    pub scan_number: u16,
    pub scanner_type: u16,
    pub motor_on: bool,
    pub laser_on: bool,
    pub frequency_locked: bool,
    pub motor_rotating_direction: u8,
    pub angle_ticks_per_rotation: u16,
    pub scan_flags: u32,
    pub mounting_yaw_angle_ticks: i16,
    pub mounting_pitch_angle_ticks: i16,
    pub mounting_roll_angle_ticks: i16,
    pub mounting_position_x: i16,
    pub mounting_position_y: i16,
    pub mounting_position_z: i16,
    pub device_id: u8,
    pub scan_start_time: crate::builtin_interfaces::msg::Time,
    pub scan_end_time: crate::builtin_interfaces::msg::Time,
    pub start_angle_ticks: i16,
    pub end_angle_ticks: i16,
    pub mirror_side: u8,
    pub mirror_tilt: i16,
    pub scan_point_list: Vec<crate::ibeo_msgs::msg::ScanPoint2208>,
}

impl ScanData2208 {
    pub const SCALA_B2: u16 = 11520;
    pub const CLOCKWISE: u8 = 0;
    pub const COUNTER_CLOCKWISE: u8 = 1;
    pub const FRONT_MIRROR: u8 = 0;
    pub const REAR_MIRROR: u8 = 1;
}

impl Default for ScanData2208 {
    fn default() -> Self {
        ScanData2208 {
            header: crate::std_msgs::msg::Header::default(),
            ibeo_header: crate::ibeo_msgs::msg::IbeoDataHeader::default(),
            scan_number: 0,
            scanner_type: 0,
            motor_on: false,
            laser_on: false,
            frequency_locked: false,
            motor_rotating_direction: 0,
            angle_ticks_per_rotation: 0,
            scan_flags: 0,
            mounting_yaw_angle_ticks: 0,
            mounting_pitch_angle_ticks: 0,
            mounting_roll_angle_ticks: 0,
            mounting_position_x: 0,
            mounting_position_y: 0,
            mounting_position_z: 0,
            device_id: 0,
            scan_start_time: crate::builtin_interfaces::msg::Time::default(),
            scan_end_time: crate::builtin_interfaces::msg::Time::default(),
            start_angle_ticks: 0,
            end_angle_ticks: 0,
            mirror_side: 0,
            mirror_tilt: 0,
            scan_point_list: Vec::new(),
        }
    }
}

impl ros2_client::Message for ScanData2208 {}
