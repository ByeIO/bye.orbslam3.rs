use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GlobalDescriptor {
    pub header: crate::std_msgs::msg::Header,
    #[serde(rename = "type")]    pub type_: i32,
    pub info: Vec<u8>,
    pub data: Vec<u8>,
}

impl Default for GlobalDescriptor {
    fn default() -> Self {
        GlobalDescriptor {
            header: crate::std_msgs::msg::Header::default(),
            type_: 0,
            info: Vec::new(),
            data: Vec::new(),
        }
    }
}

impl ros2_client::Message for GlobalDescriptor {}
