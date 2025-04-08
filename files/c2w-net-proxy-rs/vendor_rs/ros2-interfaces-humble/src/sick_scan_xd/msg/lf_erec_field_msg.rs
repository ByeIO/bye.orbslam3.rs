use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LFErecFieldMsg {
    pub version_number: u16,
    pub field_index: u8,
    pub sys_count: u32,
    pub dist_scale_factor: f32,
    pub dist_scale_offset: f32,
    pub angle_scale_factor: u32,
    pub angle_scale_offset: i32,
    pub field_result_mrs: u8,
    pub time_state: u16,
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub microsecond: u32,
}

impl Default for LFErecFieldMsg {
    fn default() -> Self {
        LFErecFieldMsg {
            version_number: 0,
            field_index: 0,
            sys_count: 0,
            dist_scale_factor: 0.0,
            dist_scale_offset: 0.0,
            angle_scale_factor: 0,
            angle_scale_offset: 0,
            field_result_mrs: 0,
            time_state: 0,
            year: 0,
            month: 0,
            day: 0,
            hour: 0,
            minute: 0,
            second: 0,
            microsecond: 0,
        }
    }
}

impl ros2_client::Message for LFErecFieldMsg {}
