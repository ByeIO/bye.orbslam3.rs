use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MotorCurrentRpt {
    pub header: crate::std_msgs::msg::Header,
    pub motor_current: u16,
}

impl Default for MotorCurrentRpt {
    fn default() -> Self {
        MotorCurrentRpt {
            header: crate::std_msgs::msg::Header::default(),
            motor_current: 0,
        }
    }
}

impl ros2_client::Message for MotorCurrentRpt {}
