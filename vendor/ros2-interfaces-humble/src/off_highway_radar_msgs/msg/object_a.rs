use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObjectA {
    pub can_id: u32,
    pub stamp: crate::builtin_interfaces::msg::Time,
    pub id: u8,
    pub position: crate::geometry_msgs::msg::Point,
    pub velocity: crate::geometry_msgs::msg::Twist,
    pub meas: bool,
    pub valid: bool,
    pub hist: bool,
}

impl Default for ObjectA {
    fn default() -> Self {
        ObjectA {
            can_id: 0,
            stamp: crate::builtin_interfaces::msg::Time::default(),
            id: 0,
            position: crate::geometry_msgs::msg::Point::default(),
            velocity: crate::geometry_msgs::msg::Twist::default(),
            meas: false,
            valid: false,
            hist: false,
        }
    }
}

impl ros2_client::Message for ObjectA {}
