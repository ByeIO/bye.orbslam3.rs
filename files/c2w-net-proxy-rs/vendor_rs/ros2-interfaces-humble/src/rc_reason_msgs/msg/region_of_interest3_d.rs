use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegionOfInterest3D {
    pub id: ::std::string::String,
    pub pose: crate::geometry_msgs::msg::PoseStamped,
    pub primitive: crate::shape_msgs::msg::SolidPrimitive,
}

impl Default for RegionOfInterest3D {
    fn default() -> Self {
        RegionOfInterest3D {
            id: ::std::string::String::new(),
            pose: crate::geometry_msgs::msg::PoseStamped::default(),
            primitive: crate::shape_msgs::msg::SolidPrimitive::default(),
        }
    }
}

impl ros2_client::Message for RegionOfInterest3D {}
