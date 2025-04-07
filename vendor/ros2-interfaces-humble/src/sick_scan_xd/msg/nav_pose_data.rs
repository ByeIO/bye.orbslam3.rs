use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NAVPoseData {
    pub header: crate::std_msgs::msg::Header,
    pub x: i32,
    pub y: i32,
    pub phi: u32,
    pub opt_pose_data_valid: u16,
    pub output_mode: u8,
    pub timestamp: u32,
    pub mean_dev: i32,
    pub nav_mode: u8,
    pub info_state: u32,
    pub quant_used_reflectors: u8,
    pub pose_valid: i8,
    pub pose_x: f32,
    pub pose_y: f32,
    pub pose_yaw: f32,
}

impl Default for NAVPoseData {
    fn default() -> Self {
        NAVPoseData {
            header: crate::std_msgs::msg::Header::default(),
            x: 0,
            y: 0,
            phi: 0,
            opt_pose_data_valid: 0,
            output_mode: 0,
            timestamp: 0,
            mean_dev: 0,
            nav_mode: 0,
            info_state: 0,
            quant_used_reflectors: 0,
            pose_valid: 0,
            pose_x: 0.0,
            pose_y: 0.0,
            pose_yaw: 0.0,
        }
    }
}

impl ros2_client::Message for NAVPoseData {}
