use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MountControl {
    pub header: crate::std_msgs::msg::Header,
    pub mode: u8,
    pub pitch: f32,
    pub roll: f32,
    pub yaw: f32,
    pub altitude: f32,
    pub latitude: f32,
    pub longitude: f32,
}

impl MountControl {
    pub const MAV_MOUNT_MODE_RETRACT: u8 = 0;
    pub const MAV_MOUNT_MODE_NEUTRAL: u8 = 1;
    pub const MAV_MOUNT_MODE_MAVLINK_TARGETING: u8 = 2;
    pub const MAV_MOUNT_MODE_RC_TARGETING: u8 = 3;
    pub const MAV_MOUNT_MODE_GPS_POINT: u8 = 4;
}

impl Default for MountControl {
    fn default() -> Self {
        MountControl {
            header: crate::std_msgs::msg::Header::default(),
            mode: 0,
            pitch: 0.0,
            roll: 0.0,
            yaw: 0.0,
            altitude: 0.0,
            latitude: 0.0,
            longitude: 0.0,
        }
    }
}

impl ros2_client::Message for MountControl {}
