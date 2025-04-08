use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VoxelGrid {
    pub header: crate::std_msgs::msg::Header,
    pub data: Vec<u32>,
    pub origin: crate::geometry_msgs::msg::Point32,
    pub resolutions: crate::geometry_msgs::msg::Vector3,
    pub size_x: u32,
    pub size_y: u32,
    pub size_z: u32,
}

impl Default for VoxelGrid {
    fn default() -> Self {
        VoxelGrid {
            header: crate::std_msgs::msg::Header::default(),
            data: Vec::new(),
            origin: crate::geometry_msgs::msg::Point32::default(),
            resolutions: crate::geometry_msgs::msg::Vector3::default(),
            size_x: 0,
            size_y: 0,
            size_z: 0,
        }
    }
}

impl ros2_client::Message for VoxelGrid {}
