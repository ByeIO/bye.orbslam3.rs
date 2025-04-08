use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DisparityImage {
    pub header: crate::std_msgs::msg::Header,
    pub image: crate::sensor_msgs::msg::Image,
    pub f: f32,
    pub t: f32,
    pub valid_window: crate::sensor_msgs::msg::RegionOfInterest,
    pub min_disparity: f32,
    pub max_disparity: f32,
    pub delta_d: f32,
}

impl Default for DisparityImage {
    fn default() -> Self {
        DisparityImage {
            header: crate::std_msgs::msg::Header::default(),
            image: crate::sensor_msgs::msg::Image::default(),
            f: 0.0,
            t: 0.0,
            valid_window: crate::sensor_msgs::msg::RegionOfInterest::default(),
            min_disparity: 0.0,
            max_disparity: 0.0,
            delta_d: 0.0,
        }
    }
}

impl ros2_client::Message for DisparityImage {}
