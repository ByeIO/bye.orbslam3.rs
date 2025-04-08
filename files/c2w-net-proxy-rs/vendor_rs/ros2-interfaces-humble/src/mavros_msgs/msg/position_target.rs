use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PositionTarget {
    pub header: crate::std_msgs::msg::Header,
    pub coordinate_frame: u8,
    pub type_mask: u16,
    pub position: crate::geometry_msgs::msg::Point,
    pub velocity: crate::geometry_msgs::msg::Vector3,
    pub acceleration_or_force: crate::geometry_msgs::msg::Vector3,
    pub yaw: f32,
    pub yaw_rate: f32,
}

impl PositionTarget {
    pub const FRAME_LOCAL_NED: u8 = 1;
    pub const FRAME_LOCAL_OFFSET_NED: u8 = 7;
    pub const FRAME_BODY_NED: u8 = 8;
    pub const FRAME_BODY_OFFSET_NED: u8 = 9;
    pub const IGNORE_PX: u16 = 1;
    pub const IGNORE_PY: u16 = 2;
    pub const IGNORE_PZ: u16 = 4;
    pub const IGNORE_VX: u16 = 8;
    pub const IGNORE_VY: u16 = 16;
    pub const IGNORE_VZ: u16 = 32;
    pub const IGNORE_AFX: u16 = 64;
    pub const IGNORE_AFY: u16 = 128;
    pub const IGNORE_AFZ: u16 = 256;
    pub const FORCE: u16 = 512;
    pub const IGNORE_YAW: u16 = 1024;
    pub const IGNORE_YAW_RATE: u16 = 2048;
}

impl Default for PositionTarget {
    fn default() -> Self {
        PositionTarget {
            header: crate::std_msgs::msg::Header::default(),
            coordinate_frame: 0,
            type_mask: 0,
            position: crate::geometry_msgs::msg::Point::default(),
            velocity: crate::geometry_msgs::msg::Vector3::default(),
            acceleration_or_force: crate::geometry_msgs::msg::Vector3::default(),
            yaw: 0.0,
            yaw_rate: 0.0,
        }
    }
}

impl ros2_client::Message for PositionTarget {}
