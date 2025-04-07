use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Network {
    #[serde(rename = "type")]    pub type_: ::std::string::String,
    pub essid: ::std::string::String,
    pub mac: ::std::string::String,
    pub mode: ::std::string::String,
    pub frequency: ::std::string::String,
    pub encryption: bool,
}

impl Default for Network {
    fn default() -> Self {
        Network {
            type_: ::std::string::String::new(),
            essid: ::std::string::String::new(),
            mac: ::std::string::String::new(),
            mode: ::std::string::String::new(),
            frequency: ::std::string::String::new(),
            encryption: false,
        }
    }
}

impl ros2_client::Message for Network {}
