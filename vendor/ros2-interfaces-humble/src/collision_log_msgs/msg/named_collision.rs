use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NamedCollision {
    pub entity0: ::std::string::String,
    pub entity1: ::std::string::String,
}

impl Default for NamedCollision {
    fn default() -> Self {
        NamedCollision {
            entity0: ::std::string::String::new(),
            entity1: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for NamedCollision {}
