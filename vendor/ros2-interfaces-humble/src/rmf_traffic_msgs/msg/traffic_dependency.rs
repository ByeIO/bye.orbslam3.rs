use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrafficDependency {
    pub dependent_checkpoint: u64,
    pub on_participant: u64,
    pub on_plan: u64,
    pub on_route: u64,
    pub on_checkpoint: u64,
}

impl Default for TrafficDependency {
    fn default() -> Self {
        TrafficDependency {
            dependent_checkpoint: 0,
            on_participant: 0,
            on_plan: 0,
            on_route: 0,
            on_checkpoint: 0,
        }
    }
}

impl ros2_client::Message for TrafficDependency {}
