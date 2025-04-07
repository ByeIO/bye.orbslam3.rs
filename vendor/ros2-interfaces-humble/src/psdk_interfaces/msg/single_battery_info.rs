use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleBatteryInfo {
    pub header: crate::std_msgs::msg::Header,
    pub battery_index: u8,
    pub voltage: f32,
    pub current: f32,
    pub full_capacity: f32,
    pub capacity_remain: f32,
    pub capacity_percentage: f32,
    pub temperature: f32,
    pub cell_count: u8,
    pub self_check_error: u32,
    pub closed_reason: u32,
    pub abnormal_comm: u16,
    pub is_embed: u16,
}

impl Default for SingleBatteryInfo {
    fn default() -> Self {
        SingleBatteryInfo {
            header: crate::std_msgs::msg::Header::default(),
            battery_index: 0,
            voltage: 0.0,
            current: 0.0,
            full_capacity: 0.0,
            capacity_remain: 0.0,
            capacity_percentage: 0.0,
            temperature: 0.0,
            cell_count: 0,
            self_check_error: 0,
            closed_reason: 0,
            abnormal_comm: 0,
            is_embed: 0,
        }
    }
}

impl ros2_client::Message for SingleBatteryInfo {}
