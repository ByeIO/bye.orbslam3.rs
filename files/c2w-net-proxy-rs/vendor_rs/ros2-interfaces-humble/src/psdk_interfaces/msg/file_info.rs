use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileInfo {
    pub name: ::std::string::String,
    #[serde(rename = "type")]    pub type_: u8,
    pub size: u32,
    pub index: u32,
    pub create_time_unix: i64,
    pub attributes: crate::psdk_interfaces::msg::FileAttributes,
    pub number_sub_files: u8,
    pub sub_files: Vec<crate::psdk_interfaces::msg::SubFileInfo>,
}

impl Default for FileInfo {
    fn default() -> Self {
        FileInfo {
            name: ::std::string::String::new(),
            type_: 0,
            size: 0,
            index: 0,
            create_time_unix: 0,
            attributes: crate::psdk_interfaces::msg::FileAttributes::default(),
            number_sub_files: 0,
            sub_files: Vec::new(),
        }
    }
}

impl ros2_client::Message for FileInfo {}
