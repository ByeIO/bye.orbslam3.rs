use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Goalpost {
    pub name: ::std::string::String,
    pub top: crate::rcss3d_agent_msgs::msg::Spherical,
}

impl Default for Goalpost {
    fn default() -> Self {
        Goalpost {
            name: ::std::string::String::new(),
            top: crate::rcss3d_agent_msgs::msg::Spherical::default(),
        }
    }
}

impl ros2_client::Message for Goalpost {}
