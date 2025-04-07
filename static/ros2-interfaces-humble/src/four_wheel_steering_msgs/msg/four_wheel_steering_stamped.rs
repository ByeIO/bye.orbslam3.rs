use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FourWheelSteeringStamped {
    pub header: crate::std_msgs::msg::Header,
    pub data: crate::four_wheel_steering_msgs::msg::FourWheelSteering,
}

impl Default for FourWheelSteeringStamped {
    fn default() -> Self {
        FourWheelSteeringStamped {
            header: crate::std_msgs::msg::Header::default(),
            data: crate::four_wheel_steering_msgs::msg::FourWheelSteering::default(),
        }
    }
}

impl ros2_client::Message for FourWheelSteeringStamped {}
