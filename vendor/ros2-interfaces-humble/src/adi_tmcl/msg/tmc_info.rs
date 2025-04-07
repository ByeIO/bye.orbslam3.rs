use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TmcInfo {
    pub header: crate::std_msgs::msg::Header,
    pub interface_name: ::std::string::String,
    pub board_voltage: f32,
    pub status_flag: i32,
    pub status: ::std::string::String,
    pub motor_num: u8,
    pub velocity: f32,
    pub position: i32,
    pub torque: i32,
}

impl Default for TmcInfo {
    fn default() -> Self {
        TmcInfo {
            header: crate::std_msgs::msg::Header::default(),
            interface_name: ::std::string::String::new(),
            board_voltage: 0.0,
            status_flag: 0,
            status: ::std::string::String::new(),
            motor_num: 0,
            velocity: 0.0,
            position: 0,
            torque: 0,
        }
    }
}

impl ros2_client::Message for TmcInfo {}
