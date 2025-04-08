use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EsrStatus5 {
    pub header: crate::std_msgs::msg::Header,
    pub canmsg: ::std::string::String,
    pub swbatt_a2d: u8,
    pub ignp_a2d: u8,
    pub temp1_a2d: u8,
    pub temp2_a2d: u8,
    pub supply_5va_a2d: u8,
    pub supply_5vdx_a2d: u8,
    pub supply_3p3v_a2d: u8,
    pub supply_10v_a2d: u8,
}

impl Default for EsrStatus5 {
    fn default() -> Self {
        EsrStatus5 {
            header: crate::std_msgs::msg::Header::default(),
            canmsg: ::std::string::String::new(),
            swbatt_a2d: 0,
            ignp_a2d: 0,
            temp1_a2d: 0,
            temp2_a2d: 0,
            supply_5va_a2d: 0,
            supply_5vdx_a2d: 0,
            supply_3p3v_a2d: 0,
            supply_10v_a2d: 0,
        }
    }
}

impl ros2_client::Message for EsrStatus5 {}
