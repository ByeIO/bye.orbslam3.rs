use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContactInformation {
    pub header: crate::std_msgs::msg::Header,
    pub position: crate::geometry_msgs::msg::Point,
    pub normal: crate::geometry_msgs::msg::Vector3,
    pub depth: f64,
    pub contact_body_1: ::std::string::String,
    pub body_type_1: u32,
    pub contact_body_2: ::std::string::String,
    pub body_type_2: u32,
}

impl ContactInformation {
    pub const ROBOT_LINK: u32 = 0;
    pub const WORLD_OBJECT: u32 = 1;
    pub const ROBOT_ATTACHED: u32 = 2;
}

impl Default for ContactInformation {
    fn default() -> Self {
        ContactInformation {
            header: crate::std_msgs::msg::Header::default(),
            position: crate::geometry_msgs::msg::Point::default(),
            normal: crate::geometry_msgs::msg::Vector3::default(),
            depth: 0.0,
            contact_body_1: ::std::string::String::new(),
            body_type_1: 0,
            contact_body_2: ::std::string::String::new(),
            body_type_2: 0,
        }
    }
}

impl ros2_client::Message for ContactInformation {}
