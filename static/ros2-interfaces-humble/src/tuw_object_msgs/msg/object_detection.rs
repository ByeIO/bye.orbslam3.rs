use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObjectDetection {
    pub header: crate::std_msgs::msg::Header,
    pub distance_min: f32,
    pub distance_max: f32,
    pub distance_max_id: f32,
    pub view_direction: crate::geometry_msgs::msg::Quaternion,
    pub fov_horizontal: f32,
    pub fov_vertical: f32,
    #[serde(rename = "type")]    pub type_: ::std::string::String,
    pub objects: Vec<crate::tuw_object_msgs::msg::ObjectWithCovariance>,
    pub sensor_type: ::std::string::String,
}

impl ObjectDetection {
    pub const OBJECT_TYPE_PERSON: &'static str = "person";
    pub const OBJECT_TYPE_OBSTACLE: &'static str = "obstacle";
    pub const OBJECT_TYPE_TRAFFIC_CONE: &'static str = "traffic_cone";
    pub const OBJECT_TYPE_DOOR: &'static str = "door";
    pub const SENSOR_TYPE_GENERIC_LASER_2D: &'static str = "laser2d";
    pub const SENSOR_TYPE_GENERIC_LASER_3D: &'static str = "laser3d";
    pub const SENSOR_TYPE_GENERIC_MONOCULAR_VISION: &'static str = "mono";
    pub const SENSOR_TYPE_GENERIC_STEREO_VISION: &'static str = "stereo";
    pub const SENSOR_TYPE_GENERIC_RGBD: &'static str = "rgbd";
}

impl Default for ObjectDetection {
    fn default() -> Self {
        ObjectDetection {
            header: crate::std_msgs::msg::Header::default(),
            distance_min: 0.0,
            distance_max: 0.0,
            distance_max_id: 0.0,
            view_direction: crate::geometry_msgs::msg::Quaternion::default(),
            fov_horizontal: 0.0,
            fov_vertical: 0.0,
            type_: ::std::string::String::new(),
            objects: Vec::new(),
            sensor_type: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ObjectDetection {}
