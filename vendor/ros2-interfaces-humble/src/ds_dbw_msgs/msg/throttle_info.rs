use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThrottleInfo {
    pub header: crate::std_msgs::msg::Header,
    pub accel_pedal_pc: f32,
    pub accel_pedal_qf: crate::ds_dbw_msgs::msg::Quality,
    pub one_pedal: crate::ds_dbw_msgs::msg::OnePedal,
    pub engine_rpm: f32,
    pub drive_mode: crate::ds_dbw_msgs::msg::DriveMode,
    pub gear_num: crate::ds_dbw_msgs::msg::GearNum,
}

impl Default for ThrottleInfo {
    fn default() -> Self {
        ThrottleInfo {
            header: crate::std_msgs::msg::Header::default(),
            accel_pedal_pc: 0.0,
            accel_pedal_qf: crate::ds_dbw_msgs::msg::Quality::default(),
            one_pedal: crate::ds_dbw_msgs::msg::OnePedal::default(),
            engine_rpm: 0.0,
            drive_mode: crate::ds_dbw_msgs::msg::DriveMode::default(),
            gear_num: crate::ds_dbw_msgs::msg::GearNum::default(),
        }
    }
}

impl ros2_client::Message for ThrottleInfo {}
