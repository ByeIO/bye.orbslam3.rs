use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompressedPointCloud2 {
    pub header: crate::std_msgs::msg::Header,
    pub height: u32,
    pub width: u32,
    pub fields: Vec<crate::sensor_msgs::msg::PointField>,
    pub is_bigendian: bool,
    pub point_step: u32,
    pub row_step: u32,
    pub compressed_data: Vec<u8>,
    pub is_dense: bool,
    pub format: ::std::string::String,
}

impl Default for CompressedPointCloud2 {
    fn default() -> Self {
        CompressedPointCloud2 {
            header: crate::std_msgs::msg::Header::default(),
            height: 0,
            width: 0,
            fields: Vec::new(),
            is_bigendian: false,
            point_step: 0,
            row_step: 0,
            compressed_data: Vec::new(),
            is_dense: false,
            format: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for CompressedPointCloud2 {}
