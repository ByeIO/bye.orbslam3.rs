use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CollisionObject {
    pub header: crate::std_msgs::msg::Header,
    pub pose: crate::geometry_msgs::msg::Pose,
    pub id: ::std::string::String,
    #[serde(rename = "type")]    pub type_: crate::object_recognition_msgs::msg::ObjectType,
    pub primitives: Vec<crate::shape_msgs::msg::SolidPrimitive>,
    pub primitive_poses: Vec<crate::geometry_msgs::msg::Pose>,
    pub meshes: Vec<crate::shape_msgs::msg::Mesh>,
    pub mesh_poses: Vec<crate::geometry_msgs::msg::Pose>,
    pub planes: Vec<crate::shape_msgs::msg::Plane>,
    pub plane_poses: Vec<crate::geometry_msgs::msg::Pose>,
    pub subframe_names: Vec<::std::string::String>,
    pub subframe_poses: Vec<crate::geometry_msgs::msg::Pose>,
    pub operation: u8,
}

impl CollisionObject {
    pub const ADD: u8 = 0;
    pub const REMOVE: u8 = 1;
    pub const APPEND: u8 = 2;
    pub const MOVE: u8 = 3;
}

impl Default for CollisionObject {
    fn default() -> Self {
        CollisionObject {
            header: crate::std_msgs::msg::Header::default(),
            pose: crate::geometry_msgs::msg::Pose::default(),
            id: ::std::string::String::new(),
            type_: crate::object_recognition_msgs::msg::ObjectType::default(),
            primitives: Vec::new(),
            primitive_poses: Vec::new(),
            meshes: Vec::new(),
            mesh_poses: Vec::new(),
            planes: Vec::new(),
            plane_poses: Vec::new(),
            subframe_names: Vec::new(),
            subframe_poses: Vec::new(),
            operation: 0,
        }
    }
}

impl ros2_client::Message for CollisionObject {}
