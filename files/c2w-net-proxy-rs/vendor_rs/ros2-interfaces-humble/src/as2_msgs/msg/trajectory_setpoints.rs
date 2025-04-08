use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrajectorySetpoints {
    pub header: crate::std_msgs::msg::Header,
    pub setpoints: Vec<crate::as2_msgs::msg::TrajectoryPoint>,
}

impl Default for TrajectorySetpoints {
    fn default() -> Self {
        TrajectorySetpoints {
            header: crate::std_msgs::msg::Header::default(),
            setpoints: Vec::new(),
        }
    }
}

impl ros2_client::Message for TrajectorySetpoints {}
