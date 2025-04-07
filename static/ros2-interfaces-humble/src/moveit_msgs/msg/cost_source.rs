use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CostSource {
    pub cost_density: f64,
    pub aabb_min: crate::geometry_msgs::msg::Vector3,
    pub aabb_max: crate::geometry_msgs::msg::Vector3,
}

impl Default for CostSource {
    fn default() -> Self {
        CostSource {
            cost_density: 0.0,
            aabb_min: crate::geometry_msgs::msg::Vector3::default(),
            aabb_max: crate::geometry_msgs::msg::Vector3::default(),
        }
    }
}

impl ros2_client::Message for CostSource {}
