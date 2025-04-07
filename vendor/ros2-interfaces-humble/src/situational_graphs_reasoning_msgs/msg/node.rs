use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Node {
    pub id: i32,
    #[serde(rename = "type")]    pub type_: ::std::string::String,
    pub attributes: Vec<crate::situational_graphs_reasoning_msgs::msg::Attribute>,
}

impl Default for Node {
    fn default() -> Self {
        Node {
            id: 0,
            type_: ::std::string::String::new(),
            attributes: Vec::new(),
        }
    }
}

impl ros2_client::Message for Node {}
