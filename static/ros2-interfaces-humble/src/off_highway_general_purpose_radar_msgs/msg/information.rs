use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Information {
    pub header: crate::std_msgs::msg::Header,
    pub sensor_type: u8,
    pub hw_temperature: f64,
    pub sensor_blind: bool,
    pub sw_fail: bool,
    pub hw_fail: bool,
    pub can_fail: bool,
    pub config_fail: bool,
    pub diag_mode: bool,
    pub dtc: u32,
    pub dtc_order_id: u8,
    pub sensor_not_safe: bool,
}

impl Default for Information {
    fn default() -> Self {
        Information {
            header: crate::std_msgs::msg::Header::default(),
            sensor_type: 0,
            hw_temperature: 0.0,
            sensor_blind: false,
            sw_fail: false,
            hw_fail: false,
            can_fail: false,
            config_fail: false,
            diag_mode: false,
            dtc: 0,
            dtc_order_id: 0,
            sensor_not_safe: false,
        }
    }
}

impl ros2_client::Message for Information {}
