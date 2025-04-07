use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TexturedMarker {
    pub header: crate::std_msgs::msg::Header,
    pub ns: ::std::string::String,
    pub id: i32,
    pub action: i32,
    pub lifetime: crate::builtin_interfaces::msg::Duration,
    pub image: crate::sensor_msgs::msg::Image,
    pub pose: crate::geometry_msgs::msg::Pose,
    pub resolution: f64,
    pub alpha: f32,
}

impl TexturedMarker {
    pub const ADD: u8 = 0;
    pub const MODIFY: u8 = 0;
    pub const DELETE: u8 = 2;
}

impl Default for TexturedMarker {
    fn default() -> Self {
        TexturedMarker {
            header: crate::std_msgs::msg::Header::default(),
            ns: ::std::string::String::new(),
            id: 0,
            action: 0,
            lifetime: crate::builtin_interfaces::msg::Duration::default(),
            image: crate::sensor_msgs::msg::Image::default(),
            pose: crate::geometry_msgs::msg::Pose::default(),
            resolution: 0.0,
            alpha: 0.0,
        }
    }
}

impl ros2_client::Message for TexturedMarker {}
