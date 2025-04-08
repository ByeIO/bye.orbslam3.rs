use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChainConnection {
    pub name: ::std::string::String,
    pub reference_interfaces: Vec<::std::string::String>,
}

impl Default for ChainConnection {
    fn default() -> Self {
        ChainConnection {
            name: ::std::string::String::new(),
            reference_interfaces: Vec::new(),
        }
    }
}

impl ros2_client::Message for ChainConnection {}
