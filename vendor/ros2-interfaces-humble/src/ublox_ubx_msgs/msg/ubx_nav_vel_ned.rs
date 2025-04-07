use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UBXNavVelNED {
    pub header: crate::std_msgs::msg::Header,
    pub itow: u32,
    pub vel_n: i32,
    pub vel_e: i32,
    pub vel_d: i32,
    pub speed: u32,
    pub g_speed: u32,
    pub heading: i32,
    pub s_acc: u32,
    pub c_acc: u32,
}

impl Default for UBXNavVelNED {
    fn default() -> Self {
        UBXNavVelNED {
            header: crate::std_msgs::msg::Header::default(),
            itow: 0,
            vel_n: 0,
            vel_e: 0,
            vel_d: 0,
            speed: 0,
            g_speed: 0,
            heading: 0,
            s_acc: 0,
            c_acc: 0,
        }
    }
}

impl ros2_client::Message for UBXNavVelNED {}
