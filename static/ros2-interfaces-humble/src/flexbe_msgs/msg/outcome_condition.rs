use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OutcomeCondition {
    pub state_name: Vec<::std::string::String>,
    pub state_outcome: Vec<::std::string::String>,
}

impl Default for OutcomeCondition {
    fn default() -> Self {
        OutcomeCondition {
            state_name: Vec::new(),
            state_outcome: Vec::new(),
        }
    }
}

impl ros2_client::Message for OutcomeCondition {}
