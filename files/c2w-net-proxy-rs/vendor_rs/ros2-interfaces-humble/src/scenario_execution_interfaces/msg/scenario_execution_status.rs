use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScenarioExecutionStatus {
    pub status: u8,
}

impl ScenarioExecutionStatus {
    pub const STOPPED: u8 = 0;
    pub const STARTING: u8 = 1;
    pub const RUNNING: u8 = 2;
    pub const SHUTTINGDOWN: u8 = 3;
    pub const ERROR: u8 = 4;
}

impl Default for ScenarioExecutionStatus {
    fn default() -> Self {
        ScenarioExecutionStatus {
            status: 0,
        }
    }
}

impl ros2_client::Message for ScenarioExecutionStatus {}
