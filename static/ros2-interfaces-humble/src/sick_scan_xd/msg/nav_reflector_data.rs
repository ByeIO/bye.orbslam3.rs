use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NAVReflectorData {
    pub cartesian_data_valid: u16,
    pub x: i32,
    pub y: i32,
    pub polar_data_valid: u16,
    pub dist: u32,
    pub phi: u32,
    pub opt_reflector_data_valid: u16,
    pub local_id: u16,
    pub global_id: u16,
    #[serde(rename = "type")]    pub type_: u8,
    pub sub_type: u16,
    pub quality: u16,
    pub timestamp: u32,
    pub size: u16,
    pub hit_count: u16,
    pub mean_echo: u16,
    pub start_index: u16,
    pub end_index: u16,
    pub pos_valid: i8,
    pub pos_x: f32,
    pub pos_y: f32,
}

impl Default for NAVReflectorData {
    fn default() -> Self {
        NAVReflectorData {
            cartesian_data_valid: 0,
            x: 0,
            y: 0,
            polar_data_valid: 0,
            dist: 0,
            phi: 0,
            opt_reflector_data_valid: 0,
            local_id: 0,
            global_id: 0,
            type_: 0,
            sub_type: 0,
            quality: 0,
            timestamp: 0,
            size: 0,
            hit_count: 0,
            mean_echo: 0,
            start_index: 0,
            end_index: 0,
            pos_valid: 0,
            pos_x: 0.0,
            pos_y: 0.0,
        }
    }
}

impl ros2_client::Message for NAVReflectorData {}
