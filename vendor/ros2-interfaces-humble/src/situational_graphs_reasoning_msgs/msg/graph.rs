use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Graph {
    pub name: ::std::string::String,
    pub nodes: Vec<crate::situational_graphs_reasoning_msgs::msg::Node>,
    pub edges: Vec<crate::situational_graphs_reasoning_msgs::msg::Edge>,
}

impl Default for Graph {
    fn default() -> Self {
        Graph {
            name: ::std::string::String::new(),
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }
}

impl ros2_client::Message for Graph {}
