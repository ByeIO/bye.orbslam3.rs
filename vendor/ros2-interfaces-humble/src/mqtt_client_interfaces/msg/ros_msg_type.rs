use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RosMsgType {
    pub md5: ::std::string::String,
    pub name: ::std::string::String,
    pub definition: ::std::string::String,
    pub stamped: bool,
}

impl Default for RosMsgType {
    fn default() -> Self {
        RosMsgType {
            md5: ::std::string::String::new(),
            name: ::std::string::String::new(),
            definition: ::std::string::String::new(),
            stamped: false,
        }
    }
}

impl ros2_client::Message for RosMsgType {}
