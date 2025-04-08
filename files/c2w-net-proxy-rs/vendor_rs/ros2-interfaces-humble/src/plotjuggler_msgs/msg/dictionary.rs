use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Dictionary {
    pub dictionary_uuid: u32,
    pub names: Vec<::std::string::String>,
}

impl Default for Dictionary {
    fn default() -> Self {
        Dictionary {
            dictionary_uuid: 0,
            names: Vec::new(),
        }
    }
}

impl ros2_client::Message for Dictionary {}
