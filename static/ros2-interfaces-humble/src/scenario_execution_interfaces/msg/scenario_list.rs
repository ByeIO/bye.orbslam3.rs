use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScenarioList {
    pub scenarios: Vec<crate::scenario_execution_interfaces::msg::Scenario>,
}

impl Default for ScenarioList {
    fn default() -> Self {
        ScenarioList {
            scenarios: Vec::new(),
        }
    }
}

impl ros2_client::Message for ScenarioList {}
