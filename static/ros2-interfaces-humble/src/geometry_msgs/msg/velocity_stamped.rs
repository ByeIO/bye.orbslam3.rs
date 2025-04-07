use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VelocityStamped {
    pub header: crate::std_msgs::msg::Header,
    pub body_frame_id: ::std::string::String,
    pub reference_frame_id: ::std::string::String,
    pub velocity: crate::geometry_msgs::msg::Twist,
}

impl Default for VelocityStamped {
    fn default() -> Self {
        VelocityStamped {
            header: crate::std_msgs::msg::Header::default(),
            body_frame_id: ::std::string::String::new(),
            reference_frame_id: ::std::string::String::new(),
            velocity: crate::geometry_msgs::msg::Twist::default(),
        }
    }
}

impl ros2_client::Message for VelocityStamped {}
