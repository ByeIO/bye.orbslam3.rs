use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Gpgsv {
    pub header: crate::std_msgs::msg::Header,
    pub message_id: ::std::string::String,
    pub n_msgs: u8,
    pub msg_number: u8,
    pub n_satellites: u8,
    pub satellites: Vec<crate::nmea_msgs::msg::GpgsvSatellite>,
}

impl Default for Gpgsv {
    fn default() -> Self {
        Gpgsv {
            header: crate::std_msgs::msg::Header::default(),
            message_id: ::std::string::String::new(),
            n_msgs: 0,
            msg_number: 0,
            n_satellites: 0,
            satellites: Vec::new(),
        }
    }
}

impl ros2_client::Message for Gpgsv {}
