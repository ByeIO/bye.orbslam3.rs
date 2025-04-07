use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClientMapVisualization {
    pub timestamp: crate::builtin_interfaces::msg::Time,
    pub visualization_id: u64,
    pub status: i32,
    pub distance_to_last_lc: f64,
    pub delay: f64,
    pub progress: f64,
    pub path_types: Vec<i32>,
}

impl ClientMapVisualization {
    pub const CLIENT_RECORDING_RECORDING_STATUS_STARTUP: i32 = -2;
    pub const CLIENT_RECORDING_RECORDING_STATUS_DELAYED: i32 = 1;
    pub const CLIENT_RECORDING_RECORDING_STATUS_OK: i32 = 2;
    pub const CLIENT_RECORDING_PATH_TYPE_ENUM_NORMAL: i32 = 0;
    pub const CLIENT_RECORDING_PATH_TYPE_ENUM_POSSIBLE_LOOP_CLOSURE: i32 = 1;
    pub const CLIENT_RECORDING_PATH_TYPE_ENUM_POSSIBLY_VISIBLE_LOOP_CLOSURE: i32 = 2;
}

impl Default for ClientMapVisualization {
    fn default() -> Self {
        ClientMapVisualization {
            timestamp: crate::builtin_interfaces::msg::Time::default(),
            visualization_id: 0,
            status: 0,
            distance_to_last_lc: 0.0,
            delay: 0.0,
            progress: 0.0,
            path_types: Vec::new(),
        }
    }
}

impl ros2_client::Message for ClientMapVisualization {}
