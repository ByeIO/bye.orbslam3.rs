use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Control {
    pub stamp: crate::builtin_interfaces::msg::Time,
    pub control_time: crate::builtin_interfaces::msg::Time,
    pub lateral: crate::autoware_control_msgs::msg::Lateral,
    pub longitudinal: crate::autoware_control_msgs::msg::Longitudinal,
}

impl Default for Control {
    fn default() -> Self {
        Control {
            stamp: crate::builtin_interfaces::msg::Time::default(),
            control_time: crate::builtin_interfaces::msg::Time::default(),
            lateral: crate::autoware_control_msgs::msg::Lateral::default(),
            longitudinal: crate::autoware_control_msgs::msg::Longitudinal::default(),
        }
    }
}

impl ros2_client::Message for Control {}
