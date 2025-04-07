use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UniversalJointVel {
    pub name: ::std::string::String,
    pub ax1: f32,
    pub ax2: f32,
}

impl Default for UniversalJointVel {
    fn default() -> Self {
        UniversalJointVel {
            name: ::std::string::String::new(),
            ax1: 0.0,
            ax2: 0.0,
        }
    }
}

impl ros2_client::Message for UniversalJointVel {}
