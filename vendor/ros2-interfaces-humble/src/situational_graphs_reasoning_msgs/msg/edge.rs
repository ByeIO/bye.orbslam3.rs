use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Edge {
    pub origin_node: i64,
    pub target_node: i64,
    pub edge_id: i64,
    pub plane_coefficients: [f64; 4],
    pub plane_information_matrix: [f64; 9],
    pub information_matrix: crate::std_msgs::msg::Float64MultiArray,
    pub measurement_transform: crate::geometry_msgs::msg::Transform,
    pub attributes: Vec<crate::situational_graphs_reasoning_msgs::msg::Attribute>,
}

impl Default for Edge {
    fn default() -> Self {
        Edge {
            origin_node: 0,
            target_node: 0,
            edge_id: 0,
            plane_coefficients: [0.0; 4],
            plane_information_matrix: [0.0; 9],
            information_matrix: crate::std_msgs::msg::Float64MultiArray::default(),
            measurement_transform: crate::geometry_msgs::msg::Transform::default(),
            attributes: Vec::new(),
        }
    }
}

impl ros2_client::Message for Edge {}
