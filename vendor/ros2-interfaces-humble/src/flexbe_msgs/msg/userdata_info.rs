use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserdataInfo {
    pub state: ::std::string::String,
    pub key: ::std::string::String,
    #[serde(rename = "type")]    pub type_: ::std::string::String,
    pub data: ::std::string::String,
}

impl Default for UserdataInfo {
    fn default() -> Self {
        UserdataInfo {
            state: ::std::string::String::new(),
            key: ::std::string::String::new(),
            type_: ::std::string::String::new(),
            data: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for UserdataInfo {}
