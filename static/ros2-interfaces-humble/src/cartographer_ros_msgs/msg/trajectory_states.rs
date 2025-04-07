use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrajectoryStates {
    pub header: crate::std_msgs::msg::Header,
    pub trajectory_id: Vec<i32>,
    pub trajectory_state: Vec<u8>,
}

impl TrajectoryStates {
    pub const ACTIVE: u8 = 0;
    pub const FINISHED: u8 = 1;
    pub const FROZEN: u8 = 2;
    pub const DELETED: u8 = 3;
}

impl Default for TrajectoryStates {
    fn default() -> Self {
        TrajectoryStates {
            header: crate::std_msgs::msg::Header::default(),
            trajectory_id: Vec::new(),
            trajectory_state: Vec::new(),
        }
    }
}

impl ros2_client::Message for TrajectoryStates {}
