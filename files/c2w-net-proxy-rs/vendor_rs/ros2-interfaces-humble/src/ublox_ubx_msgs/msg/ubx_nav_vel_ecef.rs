use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UBXNavVelECEF {
    pub header: crate::std_msgs::msg::Header,
    pub itow: u32,
    pub ecef_vx: i32,
    pub ecef_vy: i32,
    pub ecef_vz: i32,
    pub s_acc: u32,
}

impl Default for UBXNavVelECEF {
    fn default() -> Self {
        UBXNavVelECEF {
            header: crate::std_msgs::msg::Header::default(),
            itow: 0,
            ecef_vx: 0,
            ecef_vy: 0,
            ecef_vz: 0,
            s_acc: 0,
        }
    }
}

impl ros2_client::Message for UBXNavVelECEF {}
