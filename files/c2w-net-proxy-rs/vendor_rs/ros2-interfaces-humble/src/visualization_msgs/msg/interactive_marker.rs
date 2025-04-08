use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InteractiveMarker {
    pub header: crate::std_msgs::msg::Header,
    pub pose: crate::geometry_msgs::msg::Pose,
    pub name: ::std::string::String,
    pub description: ::std::string::String,
    pub scale: f32,
    pub menu_entries: Vec<crate::visualization_msgs::msg::MenuEntry>,
    pub controls: Vec<crate::visualization_msgs::msg::InteractiveMarkerControl>,
}

impl Default for InteractiveMarker {
    fn default() -> Self {
        InteractiveMarker {
            header: crate::std_msgs::msg::Header::default(),
            pose: crate::geometry_msgs::msg::Pose::default(),
            name: ::std::string::String::new(),
            description: ::std::string::String::new(),
            scale: 0.0,
            menu_entries: Vec::new(),
            controls: Vec::new(),
        }
    }
}

impl ros2_client::Message for InteractiveMarker {}
