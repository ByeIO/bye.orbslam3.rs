use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SnapshotStreamParameters {
    pub snapshot_period: f64,
    pub blackboard_data: bool,
    pub blackboard_activity: bool,
}

impl Default for SnapshotStreamParameters {
    fn default() -> Self {
        SnapshotStreamParameters {
            snapshot_period: 0.0,
            blackboard_data: false,
            blackboard_activity: false,
        }
    }
}

impl ros2_client::Message for SnapshotStreamParameters {}
