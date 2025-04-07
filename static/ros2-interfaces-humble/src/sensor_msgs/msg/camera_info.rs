use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraInfo {
    pub header: crate::std_msgs::msg::Header,
    pub height: u32,
    pub width: u32,
    pub distortion_model: ::std::string::String,
    pub d: Vec<f64>,
    pub k: [f64; 9],
    pub r: [f64; 9],
    pub p: [f64; 12],
    pub binning_x: u32,
    pub binning_y: u32,
    pub roi: crate::sensor_msgs::msg::RegionOfInterest,
}

impl Default for CameraInfo {
    fn default() -> Self {
        CameraInfo {
            header: crate::std_msgs::msg::Header::default(),
            height: 0,
            width: 0,
            distortion_model: ::std::string::String::new(),
            d: Vec::new(),
            k: [0.0; 9],
            r: [0.0; 9],
            p: [0.0; 12],
            binning_x: 0,
            binning_y: 0,
            roi: crate::sensor_msgs::msg::RegionOfInterest::default(),
        }
    }
}

impl ros2_client::Message for CameraInfo {}
