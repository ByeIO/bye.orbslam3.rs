use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FRIState {
    pub session_state: i32,
    pub connection_quality: i32,
    pub safety_state: i32,
    pub command_mode: i32,
    pub control_mode: i32,
    pub operation_mode: i32,
    pub drive_state: i32,
    pub overlay_type: i32,
    pub tracking_performance: f64,
}

impl Default for FRIState {
    fn default() -> Self {
        FRIState {
            session_state: 0,
            connection_quality: 0,
            safety_state: 0,
            command_mode: 0,
            control_mode: 0,
            operation_mode: 0,
            drive_state: 0,
            overlay_type: 0,
            tracking_performance: 0.0,
        }
    }
}

impl ros2_client::Message for FRIState {}
