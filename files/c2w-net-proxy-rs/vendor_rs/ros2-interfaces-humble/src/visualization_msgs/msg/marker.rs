use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Marker {
    pub header: crate::std_msgs::msg::Header,
    pub ns: ::std::string::String,
    pub id: i32,
    #[serde(rename = "type")]    pub type_: i32,
    pub action: i32,
    pub pose: crate::geometry_msgs::msg::Pose,
    pub scale: crate::geometry_msgs::msg::Vector3,
    pub color: crate::std_msgs::msg::ColorRGBA,
    pub lifetime: crate::builtin_interfaces::msg::Duration,
    pub frame_locked: bool,
    pub points: Vec<crate::geometry_msgs::msg::Point>,
    pub colors: Vec<crate::std_msgs::msg::ColorRGBA>,
    pub texture_resource: ::std::string::String,
    pub texture: crate::sensor_msgs::msg::CompressedImage,
    pub uv_coordinates: Vec<crate::visualization_msgs::msg::UVCoordinate>,
    pub text: ::std::string::String,
    pub mesh_resource: ::std::string::String,
    pub mesh_file: crate::visualization_msgs::msg::MeshFile,
    pub mesh_use_embedded_materials: bool,
}

impl Marker {
    pub const ARROW: i32 = 0;
    pub const CUBE: i32 = 1;
    pub const SPHERE: i32 = 2;
    pub const CYLINDER: i32 = 3;
    pub const LINE_STRIP: i32 = 4;
    pub const LINE_LIST: i32 = 5;
    pub const CUBE_LIST: i32 = 6;
    pub const SPHERE_LIST: i32 = 7;
    pub const POINTS: i32 = 8;
    pub const TEXT_VIEW_FACING: i32 = 9;
    pub const MESH_RESOURCE: i32 = 10;
    pub const TRIANGLE_LIST: i32 = 11;
    pub const ADD: i32 = 0;
    pub const MODIFY: i32 = 0;
    pub const DELETE: i32 = 2;
    pub const DELETEALL: i32 = 3;
}

impl Default for Marker {
    fn default() -> Self {
        Marker {
            header: crate::std_msgs::msg::Header::default(),
            ns: ::std::string::String::new(),
            id: 0,
            type_: 0,
            action: 0,
            pose: crate::geometry_msgs::msg::Pose::default(),
            scale: crate::geometry_msgs::msg::Vector3::default(),
            color: crate::std_msgs::msg::ColorRGBA::default(),
            lifetime: crate::builtin_interfaces::msg::Duration::default(),
            frame_locked: false,
            points: Vec::new(),
            colors: Vec::new(),
            texture_resource: ::std::string::String::new(),
            texture: crate::sensor_msgs::msg::CompressedImage::default(),
            uv_coordinates: Vec::new(),
            text: ::std::string::String::new(),
            mesh_resource: ::std::string::String::new(),
            mesh_file: crate::visualization_msgs::msg::MeshFile::default(),
            mesh_use_embedded_materials: false,
        }
    }
}

impl ros2_client::Message for Marker {}
