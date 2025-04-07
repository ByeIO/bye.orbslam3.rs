use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LinkState {
    pub link_name: ::std::string::String,
    pub pose: crate::geometry_msgs::msg::Pose,
    pub twist: crate::geometry_msgs::msg::Twist,
    pub reference_frame: ::std::string::String,
}

impl Default for LinkState {
    fn default() -> Self {
        LinkState {
            link_name: ::std::string::String::new(),
            pose: crate::geometry_msgs::msg::Pose::default(),
            twist: crate::geometry_msgs::msg::Twist::default(),
            reference_frame: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for LinkState {}
