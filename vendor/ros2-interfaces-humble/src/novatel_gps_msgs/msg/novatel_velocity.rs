use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NovatelVelocity {
    pub header: crate::std_msgs::msg::Header,
    pub novatel_msg_header: crate::novatel_gps_msgs::msg::NovatelMessageHeader,
    pub solution_status: ::std::string::String,
    pub velocity_type: ::std::string::String,
    pub latency: f32,
    pub age: f32,
    pub horizontal_speed: f64,
    pub track_ground: f64,
    pub vertical_speed: f64,
}

impl Default for NovatelVelocity {
    fn default() -> Self {
        NovatelVelocity {
            header: crate::std_msgs::msg::Header::default(),
            novatel_msg_header: crate::novatel_gps_msgs::msg::NovatelMessageHeader::default(),
            solution_status: ::std::string::String::new(),
            velocity_type: ::std::string::String::new(),
            latency: 0.0,
            age: 0.0,
            horizontal_speed: 0.0,
            track_ground: 0.0,
            vertical_speed: 0.0,
        }
    }
}

impl ros2_client::Message for NovatelVelocity {}
