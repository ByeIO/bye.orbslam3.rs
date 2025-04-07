use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RadarObject {
    pub id: i32,
    pub tracking_time: crate::builtin_interfaces::msg::Time,
    pub last_seen: crate::builtin_interfaces::msg::Time,
    pub velocity: crate::geometry_msgs::msg::TwistWithCovariance,
    pub bounding_box_center: crate::geometry_msgs::msg::Pose,
    pub bounding_box_size: crate::geometry_msgs::msg::Vector3,
    pub object_box_center: crate::geometry_msgs::msg::PoseWithCovariance,
    pub object_box_size: crate::geometry_msgs::msg::Vector3,
    pub contour_points: Vec<crate::geometry_msgs::msg::Point>,
}

impl Default for RadarObject {
    fn default() -> Self {
        RadarObject {
            id: 0,
            tracking_time: crate::builtin_interfaces::msg::Time::default(),
            last_seen: crate::builtin_interfaces::msg::Time::default(),
            velocity: crate::geometry_msgs::msg::TwistWithCovariance::default(),
            bounding_box_center: crate::geometry_msgs::msg::Pose::default(),
            bounding_box_size: crate::geometry_msgs::msg::Vector3::default(),
            object_box_center: crate::geometry_msgs::msg::PoseWithCovariance::default(),
            object_box_size: crate::geometry_msgs::msg::Vector3::default(),
            contour_points: Vec::new(),
        }
    }
}

impl ros2_client::Message for RadarObject {}
