use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LandmarkDetection {
    pub header: crate::std_msgs::msg::Header,
    pub landmark_frame_id: ::std::string::String,
    pub id: i32,
    pub size: f32,
    pub pose: crate::geometry_msgs::msg::PoseWithCovariance,
}

impl Default for LandmarkDetection {
    fn default() -> Self {
        LandmarkDetection {
            header: crate::std_msgs::msg::Header::default(),
            landmark_frame_id: ::std::string::String::new(),
            id: 0,
            size: 0.0,
            pose: crate::geometry_msgs::msg::PoseWithCovariance::default(),
        }
    }
}

impl ros2_client::Message for LandmarkDetection {}
