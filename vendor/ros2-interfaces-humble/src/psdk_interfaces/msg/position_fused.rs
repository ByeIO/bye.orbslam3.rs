use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PositionFused {
    pub header: crate::std_msgs::msg::Header,
    pub position: crate::geometry_msgs::msg::Point,
    pub x_health: u8,
    pub y_health: u8,
    pub z_health: u8,
}

impl Default for PositionFused {
    fn default() -> Self {
        PositionFused {
            header: crate::std_msgs::msg::Header::default(),
            position: crate::geometry_msgs::msg::Point::default(),
            x_health: 0,
            y_health: 0,
            z_health: 0,
        }
    }
}

impl ros2_client::Message for PositionFused {}
