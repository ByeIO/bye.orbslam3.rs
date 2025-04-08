use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UBXNavHPPosLLH {
    pub header: crate::std_msgs::msg::Header,
    pub version: u8,
    pub invalid_lon: bool,
    pub invalid_lat: bool,
    pub invalid_height: bool,
    pub invalid_hmsl: bool,
    pub invalid_lon_hp: bool,
    pub invalid_lat_hp: bool,
    pub invalid_height_hp: bool,
    pub invalid_hmsl_hp: bool,
    pub itow: u32,
    pub lon: i32,
    pub lat: i32,
    pub height: i32,
    pub hmsl: i32,
    pub lon_hp: i8,
    pub lat_hp: i8,
    pub height_hp: i8,
    pub hmsl_hp: i8,
    pub h_acc: u32,
    pub v_acc: u32,
}

impl Default for UBXNavHPPosLLH {
    fn default() -> Self {
        UBXNavHPPosLLH {
            header: crate::std_msgs::msg::Header::default(),
            version: 0,
            invalid_lon: false,
            invalid_lat: false,
            invalid_height: false,
            invalid_hmsl: false,
            invalid_lon_hp: false,
            invalid_lat_hp: false,
            invalid_height_hp: false,
            invalid_hmsl_hp: false,
            itow: 0,
            lon: 0,
            lat: 0,
            height: 0,
            hmsl: 0,
            lon_hp: 0,
            lat_hp: 0,
            height_hp: 0,
            hmsl_hp: 0,
            h_acc: 0,
            v_acc: 0,
        }
    }
}

impl ros2_client::Message for UBXNavHPPosLLH {}
