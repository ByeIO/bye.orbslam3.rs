use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EsrValid1 {
    pub header: crate::std_msgs::msg::Header,
    pub canmsg: ::std::string::String,
    pub lr_sn: u8,
    pub lr_range: f32,
    pub lr_range_rate: f32,
    pub lr_angle: f32,
    pub lr_power: i8,
}

impl Default for EsrValid1 {
    fn default() -> Self {
        EsrValid1 {
            header: crate::std_msgs::msg::Header::default(),
            canmsg: ::std::string::String::new(),
            lr_sn: 0,
            lr_range: 0.0,
            lr_range_rate: 0.0,
            lr_angle: 0.0,
            lr_power: 0,
        }
    }
}

impl ros2_client::Message for EsrValid1 {}
