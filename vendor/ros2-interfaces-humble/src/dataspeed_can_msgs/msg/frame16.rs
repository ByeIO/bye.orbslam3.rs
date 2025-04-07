use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Frame16 {
    pub header: crate::std_msgs::msg::Header,
    pub id: u32,
    pub extended: bool,
    pub fdf: bool,
    pub brs: bool,
    pub esi: bool,
    pub rtr: bool,
    pub size: u8,
    pub data: [u8; 16],
}

impl Default for Frame16 {
    fn default() -> Self {
        Frame16 {
            header: crate::std_msgs::msg::Header::default(),
            id: 0,
            extended: false,
            fdf: false,
            brs: false,
            esi: false,
            rtr: false,
            size: 0,
            data: [0; 16],
        }
    }
}

impl ros2_client::Message for Frame16 {}
