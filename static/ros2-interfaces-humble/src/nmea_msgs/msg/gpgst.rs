use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Gpgst {
    pub header: crate::std_msgs::msg::Header,
    pub message_id: ::std::string::String,
    pub utc_seconds: f64,
    pub rms: f32,
    pub semi_major_dev: f32,
    pub semi_minor_dev: f32,
    pub orientation: f32,
    pub lat_dev: f32,
    pub lon_dev: f32,
    pub alt_dev: f32,
}

impl Default for Gpgst {
    fn default() -> Self {
        Gpgst {
            header: crate::std_msgs::msg::Header::default(),
            message_id: ::std::string::String::new(),
            utc_seconds: 0.0,
            rms: 0.0,
            semi_major_dev: 0.0,
            semi_minor_dev: 0.0,
            orientation: 0.0,
            lat_dev: 0.0,
            lon_dev: 0.0,
            alt_dev: 0.0,
        }
    }
}

impl ros2_client::Message for Gpgst {}
