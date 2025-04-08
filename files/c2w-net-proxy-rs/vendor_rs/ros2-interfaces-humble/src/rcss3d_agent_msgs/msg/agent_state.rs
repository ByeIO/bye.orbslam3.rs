use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AgentState {
    pub temp: f32,
    pub battery: f32,
}

impl Default for AgentState {
    fn default() -> Self {
        AgentState {
            temp: 0.0,
            battery: 0.0,
        }
    }
}

impl ros2_client::Message for AgentState {}
