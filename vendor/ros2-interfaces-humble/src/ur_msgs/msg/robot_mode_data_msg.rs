use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RobotModeDataMsg {
    pub timestamp: u64,
    pub is_robot_connected: bool,
    pub is_real_robot_enabled: bool,
    pub is_power_on_robot: bool,
    pub is_emergency_stopped: bool,
    pub is_protective_stopped: bool,
    pub is_program_running: bool,
    pub is_program_paused: bool,
}

impl Default for RobotModeDataMsg {
    fn default() -> Self {
        RobotModeDataMsg {
            timestamp: 0,
            is_robot_connected: false,
            is_real_robot_enabled: false,
            is_power_on_robot: false,
            is_emergency_stopped: false,
            is_protective_stopped: false,
            is_program_running: false,
            is_program_paused: false,
        }
    }
}

impl ros2_client::Message for RobotModeDataMsg {}
