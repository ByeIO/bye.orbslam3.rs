use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Keyframe {
    pub header: crate::std_msgs::msg::Header,
    pub id: i32,
    #[serde(rename = "type")]    pub type_: ::std::string::String,
    pub pointcloud: crate::sensor_msgs::msg::PointCloud2,
    pub pose: crate::geometry_msgs::msg::Pose,
}

impl Default for Keyframe {
    fn default() -> Self {
        Keyframe {
            header: crate::std_msgs::msg::Header::default(),
            id: 0,
            type_: ::std::string::String::new(),
            pointcloud: crate::sensor_msgs::msg::PointCloud2::default(),
            pose: crate::geometry_msgs::msg::Pose::default(),
        }
    }
}

impl ros2_client::Message for Keyframe {}
