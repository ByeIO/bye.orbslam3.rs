use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GuiCamera {
    pub header: crate::std_msgs::msg::Header,
    pub name: ::std::string::String,
    pub view_controller: ::std::string::String,
    pub pose: crate::geometry_msgs::msg::Pose,
    pub track: crate::ros_gz_interfaces::msg::TrackVisual,
    pub projection_type: ::std::string::String,
}

impl Default for GuiCamera {
    fn default() -> Self {
        GuiCamera {
            header: crate::std_msgs::msg::Header::default(),
            name: ::std::string::String::new(),
            view_controller: ::std::string::String::new(),
            pose: crate::geometry_msgs::msg::Pose::default(),
            track: crate::ros_gz_interfaces::msg::TrackVisual::default(),
            projection_type: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GuiCamera {}
