use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PointClouds {
    pub header: crate::std_msgs::msg::Header,
    pub pointclouds: Vec<crate::sensor_msgs::msg::PointCloud2>,
}

impl Default for PointClouds {
    fn default() -> Self {
        PointClouds {
            header: crate::std_msgs::msg::Header::default(),
            pointclouds: Vec::new(),
        }
    }
}

impl ros2_client::Message for PointClouds {}
