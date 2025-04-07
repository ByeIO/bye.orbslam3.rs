use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubFileInfo {
    pub name: ::std::string::String,
    #[serde(rename = "type")]    pub type_: u8,
    pub size: u32,
    pub index: u32,
    pub create_time_unix: i64,
    pub attributes: crate::psdk_interfaces::msg::FileAttributes,
}

impl Default for SubFileInfo {
    fn default() -> Self {
        SubFileInfo {
            name: ::std::string::String::new(),
            type_: 0,
            size: 0,
            index: 0,
            create_time_unix: 0,
            attributes: crate::psdk_interfaces::msg::FileAttributes::default(),
        }
    }
}

impl ros2_client::Message for SubFileInfo {}
