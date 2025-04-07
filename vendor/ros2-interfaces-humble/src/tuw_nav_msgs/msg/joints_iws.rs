use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JointsIWS {
    pub header: crate::std_msgs::msg::Header,
    pub type_steering: ::std::string::String,
    pub type_revolute: ::std::string::String,
    pub steering: Vec<f64>,
    pub revolute: Vec<f64>,
}

impl Default for JointsIWS {
    fn default() -> Self {
        JointsIWS {
            header: crate::std_msgs::msg::Header::default(),
            type_steering: ::std::string::String::new(),
            type_revolute: ::std::string::String::new(),
            steering: Vec::new(),
            revolute: Vec::new(),
        }
    }
}

impl ros2_client::Message for JointsIWS {}
