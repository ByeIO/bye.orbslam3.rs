use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContactState {
    pub info: ::std::string::String,
    pub collision1_name: ::std::string::String,
    pub collision2_name: ::std::string::String,
    pub wrenches: Vec<crate::geometry_msgs::msg::Wrench>,
    pub total_wrench: crate::geometry_msgs::msg::Wrench,
    pub contact_positions: Vec<crate::geometry_msgs::msg::Vector3>,
    pub contact_normals: Vec<crate::geometry_msgs::msg::Vector3>,
    pub depths: Vec<f64>,
}

impl Default for ContactState {
    fn default() -> Self {
        ContactState {
            info: ::std::string::String::new(),
            collision1_name: ::std::string::String::new(),
            collision2_name: ::std::string::String::new(),
            wrenches: Vec::new(),
            total_wrench: crate::geometry_msgs::msg::Wrench::default(),
            contact_positions: Vec::new(),
            contact_normals: Vec::new(),
            depths: Vec::new(),
        }
    }
}

impl ros2_client::Message for ContactState {}
