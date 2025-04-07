use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Gprmc {
    pub header: crate::std_msgs::msg::Header,
    pub message_id: ::std::string::String,
    pub utc_seconds: f64,
    pub position_status: ::std::string::String,
    pub lat: f64,
    pub lon: f64,
    pub lat_dir: ::std::string::String,
    pub lon_dir: ::std::string::String,
    pub speed: f32,
    pub track: f32,
    pub date: ::std::string::String,
    pub mag_var: f32,
    pub mag_var_direction: ::std::string::String,
    pub mode_indicator: ::std::string::String,
}

impl Default for Gprmc {
    fn default() -> Self {
        Gprmc {
            header: crate::std_msgs::msg::Header::default(),
            message_id: ::std::string::String::new(),
            utc_seconds: 0.0,
            position_status: ::std::string::String::new(),
            lat: 0.0,
            lon: 0.0,
            lat_dir: ::std::string::String::new(),
            lon_dir: ::std::string::String::new(),
            speed: 0.0,
            track: 0.0,
            date: ::std::string::String::new(),
            mag_var: 0.0,
            mag_var_direction: ::std::string::String::new(),
            mode_indicator: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for Gprmc {}
