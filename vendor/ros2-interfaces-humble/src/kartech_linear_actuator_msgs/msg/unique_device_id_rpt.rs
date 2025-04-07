use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UniqueDeviceIdRpt {
    pub header: crate::std_msgs::msg::Header,
    pub actuator_id_first_6: u64,
    pub actuator_id_last_6: u64,
}

impl Default for UniqueDeviceIdRpt {
    fn default() -> Self {
        UniqueDeviceIdRpt {
            header: crate::std_msgs::msg::Header::default(),
            actuator_id_first_6: 0,
            actuator_id_last_6: 0,
        }
    }
}

impl ros2_client::Message for UniqueDeviceIdRpt {}
