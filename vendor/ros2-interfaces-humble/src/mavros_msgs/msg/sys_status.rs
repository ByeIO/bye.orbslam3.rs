use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SysStatus {
    pub header: crate::std_msgs::msg::Header,
    pub sensors_present: u32,
    pub sensors_enabled: u32,
    pub sensors_health: u32,
    pub load: u16,
    pub voltage_battery: u16,
    pub current_battery: i16,
    pub battery_remaining: i8,
    pub drop_rate_comm: u16,
    pub errors_comm: u16,
    pub errors_count1: u16,
    pub errors_count2: u16,
    pub errors_count3: u16,
    pub errors_count4: u16,
}

impl Default for SysStatus {
    fn default() -> Self {
        SysStatus {
            header: crate::std_msgs::msg::Header::default(),
            sensors_present: 0,
            sensors_enabled: 0,
            sensors_health: 0,
            load: 0,
            voltage_battery: 0,
            current_battery: 0,
            battery_remaining: 0,
            drop_rate_comm: 0,
            errors_comm: 0,
            errors_count1: 0,
            errors_count2: 0,
            errors_count3: 0,
            errors_count4: 0,
        }
    }
}

impl ros2_client::Message for SysStatus {}
