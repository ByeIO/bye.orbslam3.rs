use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GraphKeyframes {
    #[serde(rename = "type")]    pub type_: ::std::string::String,
    pub robot: ::std::string::String,
    pub keyframes: Vec<crate::situational_graphs_reasoning_msgs::msg::Keyframe>,
}

impl Default for GraphKeyframes {
    fn default() -> Self {
        GraphKeyframes {
            type_: ::std::string::String::new(),
            robot: ::std::string::String::new(),
            keyframes: Vec::new(),
        }
    }
}

impl ros2_client::Message for GraphKeyframes {}
