use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActuatorsPosition {
    pub header: crate::std_msgs::msg::Header,
    pub angular: crate::actuator_msgs::msg::ActuatorsAngularPosition,
    pub linear: crate::actuator_msgs::msg::ActuatorsLinearPosition,
}

impl Default for ActuatorsPosition {
    fn default() -> Self {
        ActuatorsPosition {
            header: crate::std_msgs::msg::Header::default(),
            angular: crate::actuator_msgs::msg::ActuatorsAngularPosition::default(),
            linear: crate::actuator_msgs::msg::ActuatorsLinearPosition::default(),
        }
    }
}

impl ros2_client::Message for ActuatorsPosition {}
