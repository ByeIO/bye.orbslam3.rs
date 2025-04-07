use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActuatorUniqueIdReq {
    pub header: crate::std_msgs::msg::Header,
    pub confirm: bool,
}

impl Default for ActuatorUniqueIdReq {
    fn default() -> Self {
        ActuatorUniqueIdReq {
            header: crate::std_msgs::msg::Header::default(),
            confirm: false,
        }
    }
}

impl ros2_client::Message for ActuatorUniqueIdReq {}
