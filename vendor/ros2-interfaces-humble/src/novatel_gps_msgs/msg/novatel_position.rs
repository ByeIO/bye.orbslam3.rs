use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NovatelPosition {
    pub header: crate::std_msgs::msg::Header,
    pub novatel_msg_header: crate::novatel_gps_msgs::msg::NovatelMessageHeader,
    pub solution_status: ::std::string::String,
    pub position_type: ::std::string::String,
    pub lat: f64,
    pub lon: f64,
    pub height: f64,
    pub undulation: f32,
    pub datum_id: ::std::string::String,
    pub lat_sigma: f32,
    pub lon_sigma: f32,
    pub height_sigma: f32,
    pub base_station_id: ::std::string::String,
    pub diff_age: f32,
    pub solution_age: f32,
    pub num_satellites_tracked: u8,
    pub num_satellites_used_in_solution: u8,
    pub num_gps_and_glonass_l1_used_in_solution: u8,
    pub num_gps_and_glonass_l1_and_l2_used_in_solution: u8,
    pub extended_solution_status: crate::novatel_gps_msgs::msg::NovatelExtendedSolutionStatus,
    pub signal_mask: crate::novatel_gps_msgs::msg::NovatelSignalMask,
}

impl Default for NovatelPosition {
    fn default() -> Self {
        NovatelPosition {
            header: crate::std_msgs::msg::Header::default(),
            novatel_msg_header: crate::novatel_gps_msgs::msg::NovatelMessageHeader::default(),
            solution_status: ::std::string::String::new(),
            position_type: ::std::string::String::new(),
            lat: 0.0,
            lon: 0.0,
            height: 0.0,
            undulation: 0.0,
            datum_id: ::std::string::String::new(),
            lat_sigma: 0.0,
            lon_sigma: 0.0,
            height_sigma: 0.0,
            base_station_id: ::std::string::String::new(),
            diff_age: 0.0,
            solution_age: 0.0,
            num_satellites_tracked: 0,
            num_satellites_used_in_solution: 0,
            num_gps_and_glonass_l1_used_in_solution: 0,
            num_gps_and_glonass_l1_and_l2_used_in_solution: 0,
            extended_solution_status: crate::novatel_gps_msgs::msg::NovatelExtendedSolutionStatus::default(),
            signal_mask: crate::novatel_gps_msgs::msg::NovatelSignalMask::default(),
        }
    }
}

impl ros2_client::Message for NovatelPosition {}
