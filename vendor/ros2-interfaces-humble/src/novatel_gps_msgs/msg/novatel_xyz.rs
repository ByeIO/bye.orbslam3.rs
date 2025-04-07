use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NovatelXYZ {
    pub header: crate::std_msgs::msg::Header,
    pub novatel_msg_header: crate::novatel_gps_msgs::msg::NovatelMessageHeader,
    pub solution_status: ::std::string::String,
    pub position_type: ::std::string::String,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub x_sigma: f32,
    pub y_sigma: f32,
    pub z_sigma: f32,
    pub velocity_solution_status: ::std::string::String,
    pub velocity_type: ::std::string::String,
    pub x_vel: f64,
    pub y_vel: f64,
    pub z_vel: f64,
    pub x_vel_sigma: f32,
    pub y_vel_sigma: f32,
    pub z_vel_sigma: f32,
    pub base_station_id: ::std::string::String,
    pub velocity_latency: f32,
    pub diff_age: f32,
    pub solution_age: f32,
    pub num_satellites_tracked: u8,
    pub num_satellites_used_in_solution: u8,
    pub num_gps_and_glonass_l1_used_in_solution: u8,
    pub num_gps_and_glonass_l1_and_l2_used_in_solution: u8,
    pub extended_solution_status: crate::novatel_gps_msgs::msg::NovatelExtendedSolutionStatus,
    pub signal_mask: crate::novatel_gps_msgs::msg::NovatelSignalMask,
}

impl Default for NovatelXYZ {
    fn default() -> Self {
        NovatelXYZ {
            header: crate::std_msgs::msg::Header::default(),
            novatel_msg_header: crate::novatel_gps_msgs::msg::NovatelMessageHeader::default(),
            solution_status: ::std::string::String::new(),
            position_type: ::std::string::String::new(),
            x: 0.0,
            y: 0.0,
            z: 0.0,
            x_sigma: 0.0,
            y_sigma: 0.0,
            z_sigma: 0.0,
            velocity_solution_status: ::std::string::String::new(),
            velocity_type: ::std::string::String::new(),
            x_vel: 0.0,
            y_vel: 0.0,
            z_vel: 0.0,
            x_vel_sigma: 0.0,
            y_vel_sigma: 0.0,
            z_vel_sigma: 0.0,
            base_station_id: ::std::string::String::new(),
            velocity_latency: 0.0,
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

impl ros2_client::Message for NovatelXYZ {}
