use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CartesianTrajectory {
    pub header: crate::std_msgs::msg::Header,
    pub tracked_frame: ::std::string::String,
    pub points: Vec<crate::moveit_msgs::msg::CartesianTrajectoryPoint>,
}

impl Default for CartesianTrajectory {
    fn default() -> Self {
        CartesianTrajectory {
            header: crate::std_msgs::msg::Header::default(),
            tracked_frame: ::std::string::String::new(),
            points: Vec::new(),
        }
    }
}

impl ros2_client::Message for CartesianTrajectory {}
