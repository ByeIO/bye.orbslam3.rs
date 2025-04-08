use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Trajectory {
    pub header: crate::std_msgs::msg::Header,
    pub points: Vec<crate::autoware_planning_msgs::msg::TrajectoryPoint>,
}

impl Default for Trajectory {
    fn default() -> Self {
        Trajectory {
            header: crate::std_msgs::msg::Header::default(),
            points: Vec::new(),
        }
    }
}

impl ros2_client::Message for Trajectory {}
