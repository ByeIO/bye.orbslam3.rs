use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AckermannDriveStamped {
    pub header: crate::std_msgs::msg::Header,
    pub drive: crate::ackermann_msgs::msg::AckermannDrive,
}

impl Default for AckermannDriveStamped {
    fn default() -> Self {
        AckermannDriveStamped {
            header: crate::std_msgs::msg::Header::default(),
            drive: crate::ackermann_msgs::msg::AckermannDrive::default(),
        }
    }
}

impl ros2_client::Message for AckermannDriveStamped {}
