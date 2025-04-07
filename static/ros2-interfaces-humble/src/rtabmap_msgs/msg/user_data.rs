use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserData {
    pub header: crate::std_msgs::msg::Header,
    pub rows: u32,
    pub cols: u32,
    #[serde(rename = "type")]    pub type_: u32,
    pub data: Vec<u8>,
}

impl Default for UserData {
    fn default() -> Self {
        UserData {
            header: crate::std_msgs::msg::Header::default(),
            rows: 0,
            cols: 0,
            type_: 0,
            data: Vec::new(),
        }
    }
}

impl ros2_client::Message for UserData {}
