use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Payload {
    #[serde(rename = "type")]    pub type_: u8,
    pub topic: ::std::string::String,
    pub message_type: ::std::string::String,
    pub data: Vec<u8>,
}

impl Payload {
    pub const PAYLOAD_TYPE_SERIALIZED_MESSAGE: u8 = 1;
}

impl Default for Payload {
    fn default() -> Self {
        Payload {
            type_: 0,
            topic: ::std::string::String::new(),
            message_type: ::std::string::String::new(),
            data: Vec::new(),
        }
    }
}

impl ros2_client::Message for Payload {}
