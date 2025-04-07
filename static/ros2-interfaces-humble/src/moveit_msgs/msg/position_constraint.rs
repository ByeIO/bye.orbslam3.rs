use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PositionConstraint {
    pub header: crate::std_msgs::msg::Header,
    pub link_name: ::std::string::String,
    pub target_point_offset: crate::geometry_msgs::msg::Vector3,
    pub constraint_region: crate::moveit_msgs::msg::BoundingVolume,
    pub weight: f64,
}

impl Default for PositionConstraint {
    fn default() -> Self {
        PositionConstraint {
            header: crate::std_msgs::msg::Header::default(),
            link_name: ::std::string::String::new(),
            target_point_offset: crate::geometry_msgs::msg::Vector3::default(),
            constraint_region: crate::moveit_msgs::msg::BoundingVolume::default(),
            weight: 0.0,
        }
    }
}

impl ros2_client::Message for PositionConstraint {}
