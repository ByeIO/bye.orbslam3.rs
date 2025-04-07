use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Gpgga {
    pub header: crate::std_msgs::msg::Header,
    pub message_id: ::std::string::String,
    pub utc_seconds: f64,
    pub lat: f64,
    pub lon: f64,
    pub lat_dir: ::std::string::String,
    pub lon_dir: ::std::string::String,
    pub gps_qual: u32,
    pub num_sats: u32,
    pub hdop: f32,
    pub alt: f32,
    pub altitude_units: ::std::string::String,
    pub undulation: f32,
    pub undulation_units: ::std::string::String,
    pub diff_age: u32,
    pub station_id: ::std::string::String,
}

impl Default for Gpgga {
    fn default() -> Self {
        Gpgga {
            header: crate::std_msgs::msg::Header::default(),
            message_id: ::std::string::String::new(),
            utc_seconds: 0.0,
            lat: 0.0,
            lon: 0.0,
            lat_dir: ::std::string::String::new(),
            lon_dir: ::std::string::String::new(),
            gps_qual: 0,
            num_sats: 0,
            hdop: 0.0,
            alt: 0.0,
            altitude_units: ::std::string::String::new(),
            undulation: 0.0,
            undulation_units: ::std::string::String::new(),
            diff_age: 0,
            station_id: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for Gpgga {}
