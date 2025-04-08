use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Object {
    pub ids: Vec<i32>,
    pub shape: i32,
    pub shape_variables: Vec<f64>,
    pub ids_confidence: Vec<f64>,
    pub pose: crate::geometry_msgs::msg::Pose,
    pub twist: crate::geometry_msgs::msg::Twist,
}

impl Object {
    pub const SHAPE_UNKOWN: i32 = 0;
    pub const SHAPE_POINT: i32 = 1;
    pub const SHAPE_CIRCLE: i32 = 2;
    pub const SHAPE_ELLIPSE: i32 = 3;
    pub const SHAPE_SQUARE: i32 = 4;
    pub const SHAPE_RECTANGLE: i32 = 5;
    pub const SHAPE_HULL: i32 = 6;
    pub const SHAPE_LINE: i32 = 7;
    pub const SHAPE_SPHERE: i32 = 102;
    pub const SHAPE_ELLIPSOID: i32 = 103;
    pub const SHAPE_CUBE: i32 = 104;
    pub const SHAPE_BOX: i32 = 105;
    pub const SHAPE_MESH: i32 = 106;
    pub const SHAPE_TRAFFIC_CONE: i32 = 201;
    pub const SHAPE_DOOR: i32 = 202;
    pub const SHAPE_MAP_DOOR: i32 = 212;
    pub const SHAPE_PERSON: i32 = 203;
    pub const SHAPE_VEHICLE: i32 = 204;
    pub const SHAPE_FIDUCIAL: i32 = 205;
    pub const SHAPE_CONE: i32 = 206;
}

impl Default for Object {
    fn default() -> Self {
        Object {
            ids: Vec::new(),
            shape: 0,
            shape_variables: Vec::new(),
            ids_confidence: Vec::new(),
            pose: crate::geometry_msgs::msg::Pose::default(),
            twist: crate::geometry_msgs::msg::Twist::default(),
        }
    }
}

impl ros2_client::Message for Object {}
