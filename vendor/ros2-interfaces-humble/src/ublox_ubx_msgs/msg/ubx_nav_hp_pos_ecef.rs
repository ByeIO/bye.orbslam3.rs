use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UBXNavHPPosECEF {
    pub header: crate::std_msgs::msg::Header,
    pub version: u8,
    pub itow: u32,
    pub ecef_x: i32,
    pub ecef_y: i32,
    pub ecef_z: i32,
    pub ecef_x_hp: i8,
    pub ecef_y_hp: i8,
    pub ecef_z_hp: i8,
    pub invalid_ecef_x: bool,
    pub invalid_ecef_y: bool,
    pub invalid_ecef_z: bool,
    pub invalid_ecef_x_hp: bool,
    pub invalid_ecef_y_hp: bool,
    pub invalid_ecef_z_hp: bool,
    pub p_acc: u32,
}

impl Default for UBXNavHPPosECEF {
    fn default() -> Self {
        UBXNavHPPosECEF {
            header: crate::std_msgs::msg::Header::default(),
            version: 0,
            itow: 0,
            ecef_x: 0,
            ecef_y: 0,
            ecef_z: 0,
            ecef_x_hp: 0,
            ecef_y_hp: 0,
            ecef_z_hp: 0,
            invalid_ecef_x: false,
            invalid_ecef_y: false,
            invalid_ecef_z: false,
            invalid_ecef_x_hp: false,
            invalid_ecef_y_hp: false,
            invalid_ecef_z_hp: false,
            p_acc: 0,
        }
    }
}

impl ros2_client::Message for UBXNavHPPosECEF {}
