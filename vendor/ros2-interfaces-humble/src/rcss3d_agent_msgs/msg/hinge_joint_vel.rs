use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HingeJointVel {
    pub name: ::std::string::String,
    pub ax: f32,
}

impl Default for HingeJointVel {
    fn default() -> Self {
        HingeJointVel {
            name: ::std::string::String::new(),
            ax: 0.0,
        }
    }
}

impl ros2_client::Message for HingeJointVel {}
