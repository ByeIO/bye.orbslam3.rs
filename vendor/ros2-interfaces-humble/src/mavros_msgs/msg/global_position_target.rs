use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GlobalPositionTarget {
    pub header: crate::std_msgs::msg::Header,
    pub coordinate_frame: u8,
    pub type_mask: u16,
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f32,
    pub velocity: crate::geometry_msgs::msg::Vector3,
    pub acceleration_or_force: crate::geometry_msgs::msg::Vector3,
    pub yaw: f32,
    pub yaw_rate: f32,
}

impl GlobalPositionTarget {
    pub const FRAME_GLOBAL_INT: u8 = 5;
    pub const FRAME_GLOBAL_REL_ALT: u8 = 6;
    pub const FRAME_GLOBAL_TERRAIN_ALT: u8 = 11;
    pub const IGNORE_LATITUDE: u16 = 1;
    pub const IGNORE_LONGITUDE: u16 = 2;
    pub const IGNORE_ALTITUDE: u16 = 4;
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

impl Default for GlobalPositionTarget {
    fn default() -> Self {
        GlobalPositionTarget {
            header: crate::std_msgs::msg::Header::default(),
            coordinate_frame: 0,
            type_mask: 0,
            latitude: 0.0,
            longitude: 0.0,
            altitude: 0.0,
            velocity: crate::geometry_msgs::msg::Vector3::default(),
            acceleration_or_force: crate::geometry_msgs::msg::Vector3::default(),
            yaw: 0.0,
            yaw_rate: 0.0,
        }
    }
}

impl ros2_client::Message for GlobalPositionTarget {}
