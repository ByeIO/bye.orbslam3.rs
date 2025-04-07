use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HostVehicleState2805 {
    pub header: crate::std_msgs::msg::Header,
    pub ibeo_header: crate::ibeo_msgs::msg::IbeoDataHeader,
    pub timestamp: crate::builtin_interfaces::msg::Time,
    pub scan_number: u16,
    pub error_flags: u16,
    pub longitudinal_velocity: f64,
    pub steering_wheel_angle: f64,
    pub front_wheel_angle: f64,
    pub x_position: i32,
    pub y_position: i32,
    pub course_angle: i16,
    pub time_difference: u16,
    pub x_difference: i16,
    pub y_difference: i16,
    pub heading_difference: i16,
    pub current_yaw_rate: i16,
}

impl Default for HostVehicleState2805 {
    fn default() -> Self {
        HostVehicleState2805 {
            header: crate::std_msgs::msg::Header::default(),
            ibeo_header: crate::ibeo_msgs::msg::IbeoDataHeader::default(),
            timestamp: crate::builtin_interfaces::msg::Time::default(),
            scan_number: 0,
            error_flags: 0,
            longitudinal_velocity: 0.0,
            steering_wheel_angle: 0.0,
            front_wheel_angle: 0.0,
            x_position: 0,
            y_position: 0,
            course_angle: 0,
            time_difference: 0,
            x_difference: 0,
            y_difference: 0,
            heading_difference: 0,
            current_yaw_rate: 0,
        }
    }
}

impl ros2_client::Message for HostVehicleState2805 {}
