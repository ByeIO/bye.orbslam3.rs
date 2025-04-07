use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SteeringReport {
    pub header: crate::std_msgs::msg::Header,
    pub steering_wheel_angle: f32,
    pub steering_column_torque: f32,
    pub cmd: f32,
    pub cmd_type: u8,
    pub limiting_value: bool,
    pub limiting_rate: bool,
    pub external_control: bool,
    pub ready: bool,
    pub enabled: bool,
    pub override_active: bool,
    pub override_other: bool,
    pub override_latched: bool,
    pub timeout: bool,
    pub fault: bool,
    pub bad_crc: bool,
    pub bad_rc: bool,
    pub degraded: bool,
    pub limit_rate: f32,
    pub limit_value: f32,
    pub cmd_src: crate::ds_dbw_msgs::msg::CmdSrc,
}

impl Default for SteeringReport {
    fn default() -> Self {
        SteeringReport {
            header: crate::std_msgs::msg::Header::default(),
            steering_wheel_angle: 0.0,
            steering_column_torque: 0.0,
            cmd: 0.0,
            cmd_type: 0,
            limiting_value: false,
            limiting_rate: false,
            external_control: false,
            ready: false,
            enabled: false,
            override_active: false,
            override_other: false,
            override_latched: false,
            timeout: false,
            fault: false,
            bad_crc: false,
            bad_rc: false,
            degraded: false,
            limit_rate: 0.0,
            limit_value: 0.0,
            cmd_src: crate::ds_dbw_msgs::msg::CmdSrc::default(),
        }
    }
}

impl ros2_client::Message for SteeringReport {}
