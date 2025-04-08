use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EsrStatus3 {
    pub header: crate::std_msgs::msg::Header,
    pub canmsg: ::std::string::String,
    pub interface_version: u8,
    pub hw_version: u8,
    pub sw_version_host: ::std::string::String,
    pub serial_num: ::std::string::String,
    pub sw_version_pld: u8,
}

impl Default for EsrStatus3 {
    fn default() -> Self {
        EsrStatus3 {
            header: crate::std_msgs::msg::Header::default(),
            canmsg: ::std::string::String::new(),
            interface_version: 0,
            hw_version: 0,
            sw_version_host: ::std::string::String::new(),
            serial_num: ::std::string::String::new(),
            sw_version_pld: 0,
        }
    }
}

impl ros2_client::Message for EsrStatus3 {}
