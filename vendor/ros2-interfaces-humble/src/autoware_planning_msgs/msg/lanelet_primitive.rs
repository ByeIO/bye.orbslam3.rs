use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LaneletPrimitive {
    pub id: i64,
    pub primitive_type: ::std::string::String,
}

impl Default for LaneletPrimitive {
    fn default() -> Self {
        LaneletPrimitive {
            id: 0,
            primitive_type: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for LaneletPrimitive {}
