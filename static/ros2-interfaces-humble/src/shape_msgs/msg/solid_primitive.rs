use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SolidPrimitive {
    #[serde(rename = "type")]    pub type_: u8,
    pub dimensions: Vec<f64>,
    pub polygon: crate::geometry_msgs::msg::Polygon,
}

impl SolidPrimitive {
    pub const BOX: u8 = 1;
    pub const SPHERE: u8 = 2;
    pub const CYLINDER: u8 = 3;
    pub const CONE: u8 = 4;
    pub const PRISM: u8 = 5;
    pub const BOX_X: u8 = 0;
    pub const BOX_Y: u8 = 1;
    pub const BOX_Z: u8 = 2;
    pub const SPHERE_RADIUS: u8 = 0;
    pub const CYLINDER_HEIGHT: u8 = 0;
    pub const CYLINDER_RADIUS: u8 = 1;
    pub const CONE_HEIGHT: u8 = 0;
    pub const CONE_RADIUS: u8 = 1;
    pub const PRISM_HEIGHT: u8 = 0;
}

impl Default for SolidPrimitive {
    fn default() -> Self {
        SolidPrimitive {
            type_: 0,
            dimensions: Vec::new(),
            polygon: crate::geometry_msgs::msg::Polygon::default(),
        }
    }
}

impl ros2_client::Message for SolidPrimitive {}
