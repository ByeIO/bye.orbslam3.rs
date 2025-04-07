use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccessPoint {
    pub header: crate::std_msgs::msg::Header,
    pub essid: ::std::string::String,
    pub macaddr: ::std::string::String,
    pub signal: i32,
    pub noise: i32,
    pub snr: i32,
    pub channel: i32,
    pub rate: ::std::string::String,
    pub tx_power: ::std::string::String,
    pub quality: i32,
}

impl Default for AccessPoint {
    fn default() -> Self {
        AccessPoint {
            header: crate::std_msgs::msg::Header::default(),
            essid: ::std::string::String::new(),
            macaddr: ::std::string::String::new(),
            signal: 0,
            noise: 0,
            snr: 0,
            channel: 0,
            rate: ::std::string::String::new(),
            tx_power: ::std::string::String::new(),
            quality: 0,
        }
    }
}

impl ros2_client::Message for AccessPoint {}
