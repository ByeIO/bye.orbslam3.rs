use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldLine {
    pub start: crate::rcss3d_agent_msgs::msg::Spherical,
    pub end: crate::rcss3d_agent_msgs::msg::Spherical,
}

impl Default for FieldLine {
    fn default() -> Self {
        FieldLine {
            start: crate::rcss3d_agent_msgs::msg::Spherical::default(),
            end: crate::rcss3d_agent_msgs::msg::Spherical::default(),
        }
    }
}

impl ros2_client::Message for FieldLine {}
