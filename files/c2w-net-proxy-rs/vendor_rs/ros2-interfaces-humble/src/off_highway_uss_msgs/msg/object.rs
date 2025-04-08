use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Object {
    pub header: crate::std_msgs::msg::Header,
    pub id: u8,
    pub position_first: crate::geometry_msgs::msg::Point,
    pub exist_probability: f64,
    pub position_second: crate::geometry_msgs::msg::Point,
    pub object_type: u8,
}

impl Object {
    pub const TYPE_NONE: u8 = 0;
    pub const TYPE_POINT: u8 = 1;
    pub const TYPE_LINE: u8 = 2;
    pub const TYPE_SNA: u8 = 3;
}

impl Default for Object {
    fn default() -> Self {
        Object {
            header: crate::std_msgs::msg::Header::default(),
            id: 0,
            position_first: crate::geometry_msgs::msg::Point::default(),
            exist_probability: 0.0,
            position_second: crate::geometry_msgs::msg::Point::default(),
            object_type: 0,
        }
    }
}

impl ros2_client::Message for Object {}
