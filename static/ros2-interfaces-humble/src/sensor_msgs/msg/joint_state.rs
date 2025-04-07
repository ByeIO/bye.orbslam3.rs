use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JointState {
    pub header: crate::std_msgs::msg::Header,
    pub name: Vec<::std::string::String>,
    pub position: Vec<f64>,
    pub velocity: Vec<f64>,
    pub effort: Vec<f64>,
}

impl Default for JointState {
    fn default() -> Self {
        JointState {
            header: crate::std_msgs::msg::Header::default(),
            name: Vec::new(),
            position: Vec::new(),
            velocity: Vec::new(),
            effort: Vec::new(),
        }
    }
}

impl ros2_client::Message for JointState {}
