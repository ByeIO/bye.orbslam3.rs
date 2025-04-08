use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EsrValid2 {
    pub header: crate::std_msgs::msg::Header,
    pub canmsg: ::std::string::String,
    pub mr_sn: u8,
    pub mr_range: f32,
    pub mr_range_rate: f32,
    pub mr_angle: f32,
    pub mr_power: i8,
}

impl Default for EsrValid2 {
    fn default() -> Self {
        EsrValid2 {
            header: crate::std_msgs::msg::Header::default(),
            canmsg: ::std::string::String::new(),
            mr_sn: 0,
            mr_range: 0.0,
            mr_range_rate: 0.0,
            mr_angle: 0.0,
            mr_power: 0,
        }
    }
}

impl ros2_client::Message for EsrValid2 {}
