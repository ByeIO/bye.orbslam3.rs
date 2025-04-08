use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GimbalStatus {
    pub header: crate::std_msgs::msg::Header,
    pub mount_status: u32,
    pub is_busy: u32,
    pub pitch_limited: u32,
    pub roll_limited: u32,
    pub yaw_limited: u32,
    pub calibrating: u32,
    pub prev_calibration_result: u32,
    pub installed_direction: u32,
    pub disabled_mvo: u32,
    pub gear_show_unable: u32,
    pub gyro_falut: u32,
    pub esc_pitch_status: u32,
    pub esc_roll_status: u32,
    pub esc_yaw_status: u32,
    pub drone_data_recv: u32,
    pub init_unfinished: u32,
    pub fw_updating: u32,
}

impl Default for GimbalStatus {
    fn default() -> Self {
        GimbalStatus {
            header: crate::std_msgs::msg::Header::default(),
            mount_status: 0,
            is_busy: 0,
            pitch_limited: 0,
            roll_limited: 0,
            yaw_limited: 0,
            calibrating: 0,
            prev_calibration_result: 0,
            installed_direction: 0,
            disabled_mvo: 0,
            gear_show_unable: 0,
            gyro_falut: 0,
            esc_pitch_status: 0,
            esc_roll_status: 0,
            esc_yaw_status: 0,
            drone_data_recv: 0,
            init_unfinished: 0,
            fw_updating: 0,
        }
    }
}

impl ros2_client::Message for GimbalStatus {}
