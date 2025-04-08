use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RTKYaw {
    pub header: crate::std_msgs::msg::Header,
    pub yaw: u16,
}

impl Default for RTKYaw {
    fn default() -> Self {
        RTKYaw {
            header: crate::std_msgs::msg::Header::default(),
            yaw: 0,
        }
    }
}

impl ros2_client::Message for RTKYaw {}
