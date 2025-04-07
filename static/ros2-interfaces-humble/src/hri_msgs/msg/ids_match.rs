use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IdsMatch {
    pub id1: ::std::string::String,
    pub id1_type: i8,
    pub id2: ::std::string::String,
    pub id2_type: i8,
    pub confidence: f32,
}

impl IdsMatch {
    pub const UNSET: i8 = 0;
    pub const PERSON: i8 = 1;
    pub const FACE: i8 = 2;
    pub const BODY: i8 = 3;
    pub const VOICE: i8 = 4;
}

impl Default for IdsMatch {
    fn default() -> Self {
        IdsMatch {
            id1: ::std::string::String::new(),
            id1_type: 0,
            id2: ::std::string::String::new(),
            id2_type: 0,
            confidence: 0.0,
        }
    }
}

impl ros2_client::Message for IdsMatch {}
