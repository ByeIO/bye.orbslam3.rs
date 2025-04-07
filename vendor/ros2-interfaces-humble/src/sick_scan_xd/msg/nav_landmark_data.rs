use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NAVLandmarkData {
    pub header: crate::std_msgs::msg::Header,
    pub landmark_filter: u8,
    pub num_reflectors: u16,
    pub reflectors: Vec<crate::sick_scan_xd::msg::NAVReflectorData>,
}

impl Default for NAVLandmarkData {
    fn default() -> Self {
        NAVLandmarkData {
            header: crate::std_msgs::msg::Header::default(),
            landmark_filter: 0,
            num_reflectors: 0,
            reflectors: Vec::new(),
        }
    }
}

impl ros2_client::Message for NAVLandmarkData {}
