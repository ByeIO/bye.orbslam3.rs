use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FiducialDetection {
    pub header: crate::std_msgs::msg::Header,
    pub camera_d: Vec<f64>,
    pub camera_k: [f64; 9],
    #[serde(rename = "type")]    pub type_: ::std::string::String,
    pub fiducial: Vec<crate::marker_msgs::msg::Fiducial>,
}

impl Default for FiducialDetection {
    fn default() -> Self {
        FiducialDetection {
            header: crate::std_msgs::msg::Header::default(),
            camera_d: Vec::new(),
            camera_k: [0.0; 9],
            type_: ::std::string::String::new(),
            fiducial: Vec::new(),
        }
    }
}

impl ros2_client::Message for FiducialDetection {}
