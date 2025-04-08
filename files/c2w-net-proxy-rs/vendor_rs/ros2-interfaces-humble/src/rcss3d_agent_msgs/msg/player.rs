use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Player {
    pub team: ::std::string::String,
    pub id: i32,
    pub head: Vec<crate::rcss3d_agent_msgs::msg::Spherical>,
    pub rlowerarm: Vec<crate::rcss3d_agent_msgs::msg::Spherical>,
    pub llowerarm: Vec<crate::rcss3d_agent_msgs::msg::Spherical>,
    pub rfoot: Vec<crate::rcss3d_agent_msgs::msg::Spherical>,
    pub lfoot: Vec<crate::rcss3d_agent_msgs::msg::Spherical>,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            team: ::std::string::String::new(),
            id: 0,
            head: Vec::new(),
            rlowerarm: Vec::new(),
            llowerarm: Vec::new(),
            rfoot: Vec::new(),
            lfoot: Vec::new(),
        }
    }
}

impl ros2_client::Message for Player {}
