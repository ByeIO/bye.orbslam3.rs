use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Observation {
    pub sensor_name: ::std::string::String,
    pub features: Vec<crate::geometry_msgs::msg::PointStamped>,
    pub ext_camera_info: crate::robot_calibration_msgs::msg::ExtendedCameraInfo,
    pub cloud: crate::sensor_msgs::msg::PointCloud2,
    pub image: crate::sensor_msgs::msg::Image,
}

impl Default for Observation {
    fn default() -> Self {
        Observation {
            sensor_name: ::std::string::String::new(),
            features: Vec::new(),
            ext_camera_info: crate::robot_calibration_msgs::msg::ExtendedCameraInfo::default(),
            cloud: crate::sensor_msgs::msg::PointCloud2::default(),
            image: crate::sensor_msgs::msg::Image::default(),
        }
    }
}

impl ros2_client::Message for Observation {}
