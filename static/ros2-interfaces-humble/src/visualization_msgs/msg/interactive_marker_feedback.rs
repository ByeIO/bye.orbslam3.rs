use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InteractiveMarkerFeedback {
    pub header: crate::std_msgs::msg::Header,
    pub client_id: ::std::string::String,
    pub marker_name: ::std::string::String,
    pub control_name: ::std::string::String,
    pub event_type: u8,
    pub pose: crate::geometry_msgs::msg::Pose,
    pub menu_entry_id: u32,
    pub mouse_point: crate::geometry_msgs::msg::Point,
    pub mouse_point_valid: bool,
}

impl InteractiveMarkerFeedback {
    pub const KEEP_ALIVE: u8 = 0;
    pub const POSE_UPDATE: u8 = 1;
    pub const MENU_SELECT: u8 = 2;
    pub const BUTTON_CLICK: u8 = 3;
    pub const MOUSE_DOWN: u8 = 4;
    pub const MOUSE_UP: u8 = 5;
}

impl Default for InteractiveMarkerFeedback {
    fn default() -> Self {
        InteractiveMarkerFeedback {
            header: crate::std_msgs::msg::Header::default(),
            client_id: ::std::string::String::new(),
            marker_name: ::std::string::String::new(),
            control_name: ::std::string::String::new(),
            event_type: 0,
            pose: crate::geometry_msgs::msg::Pose::default(),
            menu_entry_id: 0,
            mouse_point: crate::geometry_msgs::msg::Point::default(),
            mouse_point_valid: false,
        }
    }
}

impl ros2_client::Message for InteractiveMarkerFeedback {}
