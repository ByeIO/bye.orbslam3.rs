use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Flag {
    pub name: ::std::string::String,
    pub base: crate::rcss3d_agent_msgs::msg::Spherical,
}

impl Default for Flag {
    fn default() -> Self {
        Flag {
            name: ::std::string::String::new(),
            base: crate::rcss3d_agent_msgs::msg::Spherical::default(),
        }
    }
}

impl ros2_client::Message for Flag {}
