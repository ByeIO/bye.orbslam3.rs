use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Shape {
    #[serde(rename = "type")]    pub type_: u8,
    pub footprint: crate::geometry_msgs::msg::Polygon,
    pub dimensions: crate::geometry_msgs::msg::Vector3,
}

impl Shape {
    pub const BOUNDING_BOX: u8 = 0;
    pub const CYLINDER: u8 = 1;
    pub const POLYGON: u8 = 2;
}

impl Default for Shape {
    fn default() -> Self {
        Shape {
            type_: 0,
            footprint: crate::geometry_msgs::msg::Polygon::default(),
            dimensions: crate::geometry_msgs::msg::Vector3::default(),
        }
    }
}

impl ros2_client::Message for Shape {}
