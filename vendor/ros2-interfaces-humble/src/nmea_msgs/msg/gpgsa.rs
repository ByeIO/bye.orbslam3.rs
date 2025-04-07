use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Gpgsa {
    pub header: crate::std_msgs::msg::Header,
    pub message_id: ::std::string::String,
    pub auto_manual_mode: ::std::string::String,
    pub fix_mode: u8,
    pub sv_ids: Vec<u8>,
    pub pdop: f32,
    pub hdop: f32,
    pub vdop: f32,
}

impl Default for Gpgsa {
    fn default() -> Self {
        Gpgsa {
            header: crate::std_msgs::msg::Header::default(),
            message_id: ::std::string::String::new(),
            auto_manual_mode: ::std::string::String::new(),
            fix_mode: 0,
            sv_ids: Vec::new(),
            pdop: 0.0,
            hdop: 0.0,
            vdop: 0.0,
        }
    }
}

impl ros2_client::Message for Gpgsa {}
