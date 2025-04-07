use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RoomKeyframe {
    pub header: crate::std_msgs::msg::Header,
    pub id: i32,
    pub pointcloud: crate::sensor_msgs::msg::PointCloud2,
    pub pose: crate::geometry_msgs::msg::Pose,
    pub keyframes_ids: Vec<i32>,
    pub planes_ids: Vec<i32>,
}

impl Default for RoomKeyframe {
    fn default() -> Self {
        RoomKeyframe {
            header: crate::std_msgs::msg::Header::default(),
            id: 0,
            pointcloud: crate::sensor_msgs::msg::PointCloud2::default(),
            pose: crate::geometry_msgs::msg::Pose::default(),
            keyframes_ids: Vec::new(),
            planes_ids: Vec::new(),
        }
    }
}

impl ros2_client::Message for RoomKeyframe {}
