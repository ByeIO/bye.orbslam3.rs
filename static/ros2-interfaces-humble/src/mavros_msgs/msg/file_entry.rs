use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileEntry {
    pub name: ::std::string::String,
    #[serde(rename = "type")]    pub type_: u8,
    pub size: u64,
}

impl FileEntry {
    pub const TYPE_FILE: u8 = 0;
    pub const TYPE_DIRECTORY: u8 = 1;
}

impl Default for FileEntry {
    fn default() -> Self {
        FileEntry {
            name: ::std::string::String::new(),
            type_: 0,
            size: 0,
        }
    }
}

impl ros2_client::Message for FileEntry {}
