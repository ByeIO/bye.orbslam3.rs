use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActivityList {
    pub activities_array: Vec<::std::string::String>,
}

impl Default for ActivityList {
    fn default() -> Self {
        ActivityList {
            activities_array: Vec::new(),
        }
    }
}

impl ros2_client::Message for ActivityList {}
