use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Match {
    pub name: ::std::string::String,
    pub basis_nodes: Vec<crate::situational_graphs_reasoning_msgs::msg::Node>,
    pub target_nodes: Vec<crate::situational_graphs_reasoning_msgs::msg::Node>,
    pub edges: Vec<crate::situational_graphs_reasoning_msgs::msg::Edge>,
}

impl Default for Match {
    fn default() -> Self {
        Match {
            name: ::std::string::String::new(),
            basis_nodes: Vec::new(),
            target_nodes: Vec::new(),
            edges: Vec::new(),
        }
    }
}

impl ros2_client::Message for Match {}
