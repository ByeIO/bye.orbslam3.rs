use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NovatelUtmPosition {
    pub header: crate::std_msgs::msg::Header,
    pub novatel_msg_header: crate::novatel_gps_msgs::msg::NovatelMessageHeader,
    pub solution_status: ::std::string::String,
    pub position_type: ::std::string::String,
    pub lon_zone_number: u32,
    pub lat_zone_letter: ::std::string::String,
    pub northing: f64,
    pub easting: f64,
    pub height: f64,
    pub undulation: f32,
    pub datum_id: ::std::string::String,
    pub northing_sigma: f32,
    pub easting_sigma: f32,
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

impl Default for NovatelUtmPosition {
    fn default() -> Self {
        NovatelUtmPosition {
            header: crate::std_msgs::msg::Header::default(),
            novatel_msg_header: crate::novatel_gps_msgs::msg::NovatelMessageHeader::default(),
            solution_status: ::std::string::String::new(),
            position_type: ::std::string::String::new(),
            lon_zone_number: 0,
            lat_zone_letter: ::std::string::String::new(),
            northing: 0.0,
            easting: 0.0,
            height: 0.0,
            undulation: 0.0,
            datum_id: ::std::string::String::new(),
            northing_sigma: 0.0,
            easting_sigma: 0.0,
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

impl ros2_client::Message for NovatelUtmPosition {}
