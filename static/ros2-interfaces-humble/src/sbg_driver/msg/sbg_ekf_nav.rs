use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SbgEkfNav {
    pub header: crate::std_msgs::msg::Header,
    pub time_stamp: u32,
    pub velocity: crate::geometry_msgs::msg::Vector3,
    pub velocity_accuracy: crate::geometry_msgs::msg::Vector3,
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f64,
    pub undulation: f32,
    pub position_accuracy: crate::geometry_msgs::msg::Vector3,
    pub status: crate::sbg_driver::msg::SbgEkfStatus,
}

impl Default for SbgEkfNav {
    fn default() -> Self {
        SbgEkfNav {
            header: crate::std_msgs::msg::Header::default(),
            time_stamp: 0,
            velocity: crate::geometry_msgs::msg::Vector3::default(),
            velocity_accuracy: crate::geometry_msgs::msg::Vector3::default(),
            latitude: 0.0,
            longitude: 0.0,
            altitude: 0.0,
            undulation: 0.0,
            position_accuracy: crate::geometry_msgs::msg::Vector3::default(),
            status: crate::sbg_driver::msg::SbgEkfStatus::default(),
        }
    }
}

impl ros2_client::Message for SbgEkfNav {}
