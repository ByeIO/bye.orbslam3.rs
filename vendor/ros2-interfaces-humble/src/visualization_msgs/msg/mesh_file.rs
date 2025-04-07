use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MeshFile {
    pub filename: ::std::string::String,
    pub data: Vec<u8>,
}

impl Default for MeshFile {
    fn default() -> Self {
        MeshFile {
            filename: ::std::string::String::new(),
            data: Vec::new(),
        }
    }
}

impl ros2_client::Message for MeshFile {}
