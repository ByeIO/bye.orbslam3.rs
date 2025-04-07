use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Knowledge {
    pub instances: Vec<::std::string::String>,
    pub predicates: Vec<::std::string::String>,
    pub functions: Vec<::std::string::String>,
    pub goal: ::std::string::String,
}

impl Default for Knowledge {
    fn default() -> Self {
        Knowledge {
            instances: Vec::new(),
            predicates: Vec::new(),
            functions: Vec::new(),
            goal: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for Knowledge {}
