use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UBXNavPosECEF {
    pub header: crate::std_msgs::msg::Header,
    pub itow: u32,
    pub ecef_x: i32,
    pub ecef_y: i32,
    pub ecef_z: i32,
    pub p_acc: u32,
}

impl Default for UBXNavPosECEF {
    fn default() -> Self {
        UBXNavPosECEF {
            header: crate::std_msgs::msg::Header::default(),
            itow: 0,
            ecef_x: 0,
            ecef_y: 0,
            ecef_z: 0,
            p_acc: 0,
        }
    }
}

impl ros2_client::Message for UBXNavPosECEF {}
