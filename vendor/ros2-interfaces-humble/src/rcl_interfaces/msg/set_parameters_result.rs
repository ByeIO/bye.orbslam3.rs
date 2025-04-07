use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetParametersResult {
    pub successful: bool,
    pub reason: ::std::string::String,
}

impl Default for SetParametersResult {
    fn default() -> Self {
        SetParametersResult {
            successful: false,
            reason: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SetParametersResult {}
