use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HingeJointPos {
    pub name: ::std::string::String,
    pub ax: f32,
}

impl Default for HingeJointPos {
    fn default() -> Self {
        HingeJointPos {
            name: ::std::string::String::new(),
            ax: 0.0,
        }
    }
}

impl ros2_client::Message for HingeJointPos {}
