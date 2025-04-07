use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UlcReport {
    pub header: crate::std_msgs::msg::Header,
    pub cmd_type: u8,
    pub vel_ref: f32,
    pub vel_meas: f32,
    pub accel_ref: f32,
    pub accel_meas: f32,
    pub coast_decel: bool,
    pub ready: bool,
    pub enabled: bool,
    pub override_active: bool,
    pub override_latched: bool,
    pub preempted: bool,
    pub timeout: bool,
    pub bad_crc: bool,
    pub bad_rc: bool,
}

impl Default for UlcReport {
    fn default() -> Self {
        UlcReport {
            header: crate::std_msgs::msg::Header::default(),
            cmd_type: 0,
            vel_ref: 0.0,
            vel_meas: 0.0,
            accel_ref: 0.0,
            accel_meas: 0.0,
            coast_decel: false,
            ready: false,
            enabled: false,
            override_active: false,
            override_latched: false,
            preempted: false,
            timeout: false,
            bad_crc: false,
            bad_rc: false,
        }
    }
}

impl ros2_client::Message for UlcReport {}
