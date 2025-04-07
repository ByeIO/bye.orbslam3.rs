use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Shape {
    pub id: i64, // default: -1
    pub shape: u32, // default: 0
    #[serde(rename = "type")]    pub type_: u32, // default: 0
    pub poses: Vec<crate::geometry_msgs::msg::Pose>,
    pub params_poses: Vec<crate::tuw_std_msgs::msg::ParameterArray>,
    pub params: crate::tuw_std_msgs::msg::ParameterArray,
}

impl Shape {
    pub const SHAPE_NA: u32 = 0;
    pub const SHAPE_POINT: u32 = 1;
    pub const SHAPE_LINE: u32 = 2;
    pub const SHAPE_LINE_STRIP: u32 = 3;
    pub const SHAPE_CIRCLE: u32 = 4;
    pub const SHAPE_POLYGON: u32 = 5;
    pub const SHAPE_RECTANGLE: u32 = 6;
    pub const TYPE_NA: u32 = 0;
    pub const TYPE_MAP: u32 = 100;
    pub const TYPE_PLANT: u32 = 200;
    pub const TYPE_PLANT_WINE_ROW: u32 = 201;
    pub const TYPE_OBSTACLE: u32 = 300;
    pub const TYPE_OBSTACLE_HOUSE: u32 = 301;
    pub const TYPE_OBSTACLE_TREE: u32 = 302;
    pub const TYPE_TRANSIT: u32 = 400;
    pub const TYPE_TRANSIT_STREET: u32 = 401;
    pub const TYPE_TRANSIT_GRAVEL: u32 = 402;
    pub const TYPE_WALL: u32 = 500;
    pub const TYPE_WALL_ROOM: u32 = 501;
    pub const TYPE_WALL_HALLWAY: u32 = 502;
}

impl Default for Shape {
    fn default() -> Self {
        Shape {
            id: -1,
            shape: 0,
            type_: 0,
            poses: Vec::new(),
            params_poses: Vec::new(),
            params: crate::tuw_std_msgs::msg::ParameterArray::default(),
        }
    }
}

impl ros2_client::Message for Shape {}
