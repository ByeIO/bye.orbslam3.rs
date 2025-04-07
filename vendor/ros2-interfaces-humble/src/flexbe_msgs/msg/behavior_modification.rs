use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BehaviorModification {
    pub index_begin: i32,
    pub index_end: i32,
    pub new_content: ::std::string::String,
}

impl Default for BehaviorModification {
    fn default() -> Self {
        BehaviorModification {
            index_begin: 0,
            index_end: 0,
            new_content: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for BehaviorModification {}
