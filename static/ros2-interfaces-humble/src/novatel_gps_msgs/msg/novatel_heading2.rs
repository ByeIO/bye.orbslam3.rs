use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NovatelHeading2 {
    pub header: crate::std_msgs::msg::Header,
    pub novatel_msg_header: crate::novatel_gps_msgs::msg::NovatelMessageHeader,
    pub solution_status: ::std::string::String,
    pub position_type: ::std::string::String,
    pub baseline_length: f32,
    pub heading: f32,
    pub pitch: f32,
    pub heading_sigma: f32,
    pub pitch_sigma: f32,
    pub rover_station_id: ::std::string::String,
    pub master_station_id: ::std::string::String,
    pub num_satellites_tracked: u8,
    pub num_satellites_used_in_solution: u8,
    pub num_satellites_above_elevation_mask_angle: u8,
    pub num_satellites_above_elevation_mask_angle_l2: u8,
    pub solution_source: u8,
    pub extended_solution_status: crate::novatel_gps_msgs::msg::NovatelExtendedSolutionStatus,
    pub signal_mask: crate::novatel_gps_msgs::msg::NovatelSignalMask,
}

impl NovatelHeading2 {
    pub const SOURCE_PRIMARY_ANTENNA: u8 = 0;
    pub const SOURCE_SECONDARY_ANTENNA: u8 = 1;
}

impl Default for NovatelHeading2 {
    fn default() -> Self {
        NovatelHeading2 {
            header: crate::std_msgs::msg::Header::default(),
            novatel_msg_header: crate::novatel_gps_msgs::msg::NovatelMessageHeader::default(),
            solution_status: ::std::string::String::new(),
            position_type: ::std::string::String::new(),
            baseline_length: 0.0,
            heading: 0.0,
            pitch: 0.0,
            heading_sigma: 0.0,
            pitch_sigma: 0.0,
            rover_station_id: ::std::string::String::new(),
            master_station_id: ::std::string::String::new(),
            num_satellites_tracked: 0,
            num_satellites_used_in_solution: 0,
            num_satellites_above_elevation_mask_angle: 0,
            num_satellites_above_elevation_mask_angle_l2: 0,
            solution_source: 0,
            extended_solution_status: crate::novatel_gps_msgs::msg::NovatelExtendedSolutionStatus::default(),
            signal_mask: crate::novatel_gps_msgs::msg::NovatelSignalMask::default(),
        }
    }
}

impl ros2_client::Message for NovatelHeading2 {}
