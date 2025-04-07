use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Entity {
    pub id: u64,
    pub name: ::std::string::String,
    #[serde(rename = "type")]    pub type_: u8,
}

impl Entity {
    pub const NONE: u8 = 0;
    pub const LIGHT: u8 = 1;
    pub const MODEL: u8 = 2;
    pub const LINK: u8 = 3;
    pub const VISUAL: u8 = 4;
    pub const COLLISION: u8 = 5;
    pub const SENSOR: u8 = 6;
    pub const JOINT: u8 = 7;
}

impl Default for Entity {
    fn default() -> Self {
        Entity {
            id: 0,
            name: ::std::string::String::new(),
            type_: 0,
        }
    }
}

impl ros2_client::Message for Entity {}
