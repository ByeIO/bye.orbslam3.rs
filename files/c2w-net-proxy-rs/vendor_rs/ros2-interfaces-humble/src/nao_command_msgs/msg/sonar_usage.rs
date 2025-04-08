use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SonarUsage {
    pub left: bool,
    pub right: bool,
}

impl Default for SonarUsage {
    fn default() -> Self {
        SonarUsage {
            left: false,
            right: false,
        }
    }
}

impl ros2_client::Message for SonarUsage {}
