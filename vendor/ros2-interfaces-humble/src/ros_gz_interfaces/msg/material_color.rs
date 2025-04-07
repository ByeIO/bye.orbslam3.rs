use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MaterialColor {
    pub header: crate::std_msgs::msg::Header,
    pub entity: crate::ros_gz_interfaces::msg::Entity,
    pub ambient: crate::std_msgs::msg::ColorRGBA,
    pub diffuse: crate::std_msgs::msg::ColorRGBA,
    pub specular: crate::std_msgs::msg::ColorRGBA,
    pub emissive: crate::std_msgs::msg::ColorRGBA,
    pub shininess: f64,
    pub entity_match: u8,
}

impl MaterialColor {
    pub const FIRST: u8 = 0;
    pub const ALL: u8 = 1;
}

impl Default for MaterialColor {
    fn default() -> Self {
        MaterialColor {
            header: crate::std_msgs::msg::Header::default(),
            entity: crate::ros_gz_interfaces::msg::Entity::default(),
            ambient: crate::std_msgs::msg::ColorRGBA::default(),
            diffuse: crate::std_msgs::msg::ColorRGBA::default(),
            specular: crate::std_msgs::msg::ColorRGBA::default(),
            emissive: crate::std_msgs::msg::ColorRGBA::default(),
            shininess: 0.0,
            entity_match: 0,
        }
    }
}

impl ros2_client::Message for MaterialColor {}
