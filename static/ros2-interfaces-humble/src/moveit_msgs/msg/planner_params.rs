use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlannerParams {
    pub keys: Vec<::std::string::String>,
    pub values: Vec<::std::string::String>,
    pub descriptions: Vec<::std::string::String>,
}

impl Default for PlannerParams {
    fn default() -> Self {
        PlannerParams {
            keys: Vec::new(),
            values: Vec::new(),
            descriptions: Vec::new(),
        }
    }
}

impl ros2_client::Message for PlannerParams {}
