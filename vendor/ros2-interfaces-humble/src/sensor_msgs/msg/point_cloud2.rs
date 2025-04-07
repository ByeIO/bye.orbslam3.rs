use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PointCloud2 {
    pub header: crate::std_msgs::msg::Header,
    pub height: u32,
    pub width: u32,
    pub fields: Vec<crate::sensor_msgs::msg::PointField>,
    pub is_bigendian: bool,
    pub point_step: u32,
    pub row_step: u32,
    pub data: Vec<u8>,
    pub is_dense: bool,
}

impl Default for PointCloud2 {
    fn default() -> Self {
        PointCloud2 {
            header: crate::std_msgs::msg::Header::default(),
            height: 0,
            width: 0,
            fields: Vec::new(),
            is_bigendian: false,
            point_step: 0,
            row_step: 0,
            data: Vec::new(),
            is_dense: false,
        }
    }
}

impl ros2_client::Message for PointCloud2 {}
