use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BoundingVolume {
    pub primitives: Vec<crate::shape_msgs::msg::SolidPrimitive>,
    pub primitive_poses: Vec<crate::geometry_msgs::msg::Pose>,
    pub meshes: Vec<crate::shape_msgs::msg::Mesh>,
    pub mesh_poses: Vec<crate::geometry_msgs::msg::Pose>,
}

impl Default for BoundingVolume {
    fn default() -> Self {
        BoundingVolume {
            primitives: Vec::new(),
            primitive_poses: Vec::new(),
            meshes: Vec::new(),
            mesh_poses: Vec::new(),
        }
    }
}

impl ros2_client::Message for BoundingVolume {}
