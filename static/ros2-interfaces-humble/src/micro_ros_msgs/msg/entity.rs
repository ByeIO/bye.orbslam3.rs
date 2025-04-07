use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Entity {
    pub entity_type: u8,
    pub name: ::std::string::String,
    pub types: Vec<::std::string::String>,
}

impl Entity {
    pub const PUBLISHER: u8 = 0;
    pub const SUBSCRIBER: u8 = 1;
    pub const SERVICE_SERVER: u8 = 2;
    pub const SERVICE_CLIENT: u8 = 3;
}

impl Default for Entity {
    fn default() -> Self {
        Entity {
            entity_type: 0,
            name: ::std::string::String::new(),
            types: Vec::new(),
        }
    }
}

impl ros2_client::Message for Entity {}
