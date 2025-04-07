use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Object {
    pub header: crate::std_msgs::msg::Header,
    pub name: ::std::string::String,
    pub support_surface: ::std::string::String,
    pub properties: Vec<crate::grasping_msgs::msg::ObjectProperty>,
    pub point_cluster: crate::sensor_msgs::msg::PointCloud2,
    pub primitives: Vec<crate::shape_msgs::msg::SolidPrimitive>,
    pub primitive_poses: Vec<crate::geometry_msgs::msg::Pose>,
    pub meshes: Vec<crate::shape_msgs::msg::Mesh>,
    pub mesh_poses: Vec<crate::geometry_msgs::msg::Pose>,
    pub surface: crate::shape_msgs::msg::Plane,
}

impl Default for Object {
    fn default() -> Self {
        Object {
            header: crate::std_msgs::msg::Header::default(),
            name: ::std::string::String::new(),
            support_surface: ::std::string::String::new(),
            properties: Vec::new(),
            point_cluster: crate::sensor_msgs::msg::PointCloud2::default(),
            primitives: Vec::new(),
            primitive_poses: Vec::new(),
            meshes: Vec::new(),
            mesh_poses: Vec::new(),
            surface: crate::shape_msgs::msg::Plane::default(),
        }
    }
}

impl ros2_client::Message for Object {}
