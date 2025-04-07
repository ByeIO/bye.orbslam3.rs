use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RelativeObstacleInfo {
    pub header: crate::std_msgs::msg::Header,
    pub down: f32,
    pub front: f32,
    pub right: f32,
    pub back: f32,
    pub left: f32,
    pub up: f32,
    pub down_health: u8,
    pub front_health: u8,
    pub right_health: u8,
    pub back_health: u8,
    pub left_health: u8,
    pub up_health: u8,
    pub reserved: u8,
}

impl Default for RelativeObstacleInfo {
    fn default() -> Self {
        RelativeObstacleInfo {
            header: crate::std_msgs::msg::Header::default(),
            down: 0.0,
            front: 0.0,
            right: 0.0,
            back: 0.0,
            left: 0.0,
            up: 0.0,
            down_health: 0,
            front_health: 0,
            right_health: 0,
            back_health: 0,
            left_health: 0,
            up_health: 0,
            reserved: 0,
        }
    }
}

impl ros2_client::Message for RelativeObstacleInfo {}
