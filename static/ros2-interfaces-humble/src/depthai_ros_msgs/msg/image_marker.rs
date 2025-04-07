use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageMarker {
    pub header: crate::depthai_ros_msgs::msg::Header,
    pub ns: ::std::string::String,
    pub id: i32,
    #[serde(rename = "type")]    pub type_: i32,
    pub action: i32,
    pub position: crate::geometry_msgs::msg::Point,
    pub scale: f32,
    pub outline_color: crate::std_msgs::msg::ColorRGBA,
    pub filled: u8,
    pub fill_color: crate::std_msgs::msg::ColorRGBA,
    pub lifetime: crate::depthai_ros_msgs::msg::duration,
    pub points: Vec<crate::geometry_msgs::msg::Point>,
    pub outline_colors: Vec<crate::std_msgs::msg::ColorRGBA>,
    pub text: crate::std_msgs::msg::String,
}

impl ImageMarker {
    pub const CIRCLE: u8 = 0;
    pub const LINE_STRIP: u8 = 1;
    pub const LINE_LIST: u8 = 2;
    pub const POLYGON: u8 = 3;
    pub const POINTS: u8 = 4;
    pub const TEXT: u8 = 5;
    pub const ADD: u8 = 0;
    pub const REMOVE: u8 = 1;
}

impl Default for ImageMarker {
    fn default() -> Self {
        ImageMarker {
            header: crate::depthai_ros_msgs::msg::Header::default(),
            ns: ::std::string::String::new(),
            id: 0,
            type_: 0,
            action: 0,
            position: crate::geometry_msgs::msg::Point::default(),
            scale: 0.0,
            outline_color: crate::std_msgs::msg::ColorRGBA::default(),
            filled: 0,
            fill_color: crate::std_msgs::msg::ColorRGBA::default(),
            lifetime: crate::depthai_ros_msgs::msg::duration::default(),
            points: Vec::new(),
            outline_colors: Vec::new(),
            text: crate::std_msgs::msg::String::default(),
        }
    }
}

impl ros2_client::Message for ImageMarker {}
