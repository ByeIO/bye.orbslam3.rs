use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListParametersResult {
    pub names: Vec<::std::string::String>,
    pub prefixes: Vec<::std::string::String>,
}

impl Default for ListParametersResult {
    fn default() -> Self {
        ListParametersResult {
            names: Vec::new(),
            prefixes: Vec::new(),
        }
    }
}

impl ros2_client::Message for ListParametersResult {}
