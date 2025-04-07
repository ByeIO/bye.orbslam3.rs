use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScanData2202 {
    pub header: crate::std_msgs::msg::Header,
    pub ibeo_header: crate::ibeo_msgs::msg::IbeoDataHeader,
    pub scan_number: u16,
    pub scanner_status: u16,
    pub sync_phase_offset: u16,
    pub scan_start_time: crate::builtin_interfaces::msg::Time,
    pub scan_end_time: crate::builtin_interfaces::msg::Time,
    pub angle_ticks_per_rotation: u16,
    pub start_angle_ticks: i16,
    pub end_angle_ticks: i16,
    pub scan_points_count: u16,
    pub mounting_yaw_angle_ticks: i16,
    pub mounting_pitch_angle_ticks: i16,
    pub mounting_roll_angle_ticks: i16,
    pub mounting_position_x: i16,
    pub mounting_position_y: i16,
    pub mounting_position_z: i16,
    pub ground_labeled: bool,
    pub dirt_labeled: bool,
    pub rain_labeled: bool,
    pub mirror_side: u8,
    pub scan_point_list: Vec<crate::ibeo_msgs::msg::ScanPoint2202>,
}

impl ScanData2202 {
    pub const FRONT: u8 = 0;
    pub const REAR: u8 = 1;
}

impl Default for ScanData2202 {
    fn default() -> Self {
        ScanData2202 {
            header: crate::std_msgs::msg::Header::default(),
            ibeo_header: crate::ibeo_msgs::msg::IbeoDataHeader::default(),
            scan_number: 0,
            scanner_status: 0,
            sync_phase_offset: 0,
            scan_start_time: crate::builtin_interfaces::msg::Time::default(),
            scan_end_time: crate::builtin_interfaces::msg::Time::default(),
            angle_ticks_per_rotation: 0,
            start_angle_ticks: 0,
            end_angle_ticks: 0,
            scan_points_count: 0,
            mounting_yaw_angle_ticks: 0,
            mounting_pitch_angle_ticks: 0,
            mounting_roll_angle_ticks: 0,
            mounting_position_x: 0,
            mounting_position_y: 0,
            mounting_position_z: 0,
            ground_labeled: false,
            dirt_labeled: false,
            rain_labeled: false,
            mirror_side: 0,
            scan_point_list: Vec::new(),
        }
    }
}

impl ros2_client::Message for ScanData2202 {}
