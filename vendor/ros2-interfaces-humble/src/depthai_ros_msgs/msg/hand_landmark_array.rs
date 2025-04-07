use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HandLandmarkArray {
    pub header: crate::std_msgs::msg::Header,
    pub landmarks: Vec<crate::depthai_ros_msgs::msg::HandLandmark>,
}

impl Default for HandLandmarkArray {
    fn default() -> Self {
        HandLandmarkArray {
            header: crate::std_msgs::msg::Header::default(),
            landmarks: Vec::new(),
        }
    }
}

impl ros2_client::Message for HandLandmarkArray {}
