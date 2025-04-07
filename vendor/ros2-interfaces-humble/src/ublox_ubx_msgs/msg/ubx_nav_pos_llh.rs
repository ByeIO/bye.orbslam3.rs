use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UBXNavPosLLH {
    pub header: crate::std_msgs::msg::Header,
    pub itow: u32,
    pub lon: i32,
    pub lat: i32,
    pub height: i32,
    pub hmsl: i32,
    pub h_acc: u32,
    pub v_acc: u32,
}

impl Default for UBXNavPosLLH {
    fn default() -> Self {
        UBXNavPosLLH {
            header: crate::std_msgs::msg::Header::default(),
            itow: 0,
            lon: 0,
            lat: 0,
            height: 0,
            hmsl: 0,
            h_acc: 0,
            v_acc: 0,
        }
    }
}

impl ros2_client::Message for UBXNavPosLLH {}
