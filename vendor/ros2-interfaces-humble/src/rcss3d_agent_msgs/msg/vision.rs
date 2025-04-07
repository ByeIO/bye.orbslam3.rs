use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Vision {
    pub ball: Vec<crate::rcss3d_agent_msgs::msg::Ball>,
    pub field_lines: Vec<crate::rcss3d_agent_msgs::msg::FieldLine>,
    pub flags: Vec<crate::rcss3d_agent_msgs::msg::Flag>,
    pub goalposts: Vec<crate::rcss3d_agent_msgs::msg::Goalpost>,
    pub players: Vec<crate::rcss3d_agent_msgs::msg::Player>,
}

impl Default for Vision {
    fn default() -> Self {
        Vision {
            ball: Vec::new(),
            field_lines: Vec::new(),
            flags: Vec::new(),
            goalposts: Vec::new(),
            players: Vec::new(),
        }
    }
}

impl ros2_client::Message for Vision {}
