use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InteractiveMarkerControl {
    pub name: ::std::string::String,
    pub orientation: crate::geometry_msgs::msg::Quaternion,
    pub orientation_mode: u8,
    pub interaction_mode: u8,
    pub always_visible: bool,
    pub markers: Vec<crate::visualization_msgs::msg::Marker>,
    pub independent_marker_orientation: bool,
    pub description: ::std::string::String,
}

impl InteractiveMarkerControl {
    pub const INHERIT: u8 = 0;
    pub const FIXED: u8 = 1;
    pub const VIEW_FACING: u8 = 2;
    pub const NONE: u8 = 0;
    pub const MENU: u8 = 1;
    pub const BUTTON: u8 = 2;
    pub const MOVE_AXIS: u8 = 3;
    pub const MOVE_PLANE: u8 = 4;
    pub const ROTATE_AXIS: u8 = 5;
    pub const MOVE_ROTATE: u8 = 6;
    pub const MOVE_3D: u8 = 7;
    pub const ROTATE_3D: u8 = 8;
    pub const MOVE_ROTATE_3D: u8 = 9;
}

impl Default for InteractiveMarkerControl {
    fn default() -> Self {
        InteractiveMarkerControl {
            name: ::std::string::String::new(),
            orientation: crate::geometry_msgs::msg::Quaternion::default(),
            orientation_mode: 0,
            interaction_mode: 0,
            always_visible: false,
            markers: Vec::new(),
            independent_marker_orientation: false,
            description: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for InteractiveMarkerControl {}
