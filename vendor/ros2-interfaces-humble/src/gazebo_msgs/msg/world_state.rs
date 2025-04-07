use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorldState {
    pub header: crate::std_msgs::msg::Header,
    pub name: Vec<::std::string::String>,
    pub pose: Vec<crate::geometry_msgs::msg::Pose>,
    pub twist: Vec<crate::geometry_msgs::msg::Twist>,
    pub wrench: Vec<crate::geometry_msgs::msg::Wrench>,
}

impl Default for WorldState {
    fn default() -> Self {
        WorldState {
            header: crate::std_msgs::msg::Header::default(),
            name: Vec::new(),
            pose: Vec::new(),
            twist: Vec::new(),
            wrench: Vec::new(),
        }
    }
}

impl ros2_client::Message for WorldState {}
