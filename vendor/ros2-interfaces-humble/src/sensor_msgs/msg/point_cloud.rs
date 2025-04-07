use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PointCloud {
    pub header: crate::std_msgs::msg::Header,
    pub points: Vec<crate::geometry_msgs::msg::Point32>,
    pub channels: Vec<crate::sensor_msgs::msg::ChannelFloat32>,
}

impl Default for PointCloud {
    fn default() -> Self {
        PointCloud {
            header: crate::std_msgs::msg::Header::default(),
            points: Vec::new(),
            channels: Vec::new(),
        }
    }
}

impl ros2_client::Message for PointCloud {}
