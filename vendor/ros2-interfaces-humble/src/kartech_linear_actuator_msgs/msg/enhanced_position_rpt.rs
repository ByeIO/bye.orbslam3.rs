use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnhancedPositionRpt {
    pub header: crate::std_msgs::msg::Header,
    pub shaft_extension: f64,
    pub motor_overload_error: bool,
    pub clutch_overload_error: bool,
    pub motor_open_load_error: bool,
    pub clutch_open_load_error: bool,
    pub position_reach_error: bool,
    pub hardware_warning_1_error: bool,
    pub hardware_warning_2_error: bool,
    pub motor_current: u16,
}

impl Default for EnhancedPositionRpt {
    fn default() -> Self {
        EnhancedPositionRpt {
            header: crate::std_msgs::msg::Header::default(),
            shaft_extension: 0.0,
            motor_overload_error: false,
            clutch_overload_error: false,
            motor_open_load_error: false,
            clutch_open_load_error: false,
            position_reach_error: false,
            hardware_warning_1_error: false,
            hardware_warning_2_error: false,
            motor_current: 0,
        }
    }
}

impl ros2_client::Message for EnhancedPositionRpt {}
