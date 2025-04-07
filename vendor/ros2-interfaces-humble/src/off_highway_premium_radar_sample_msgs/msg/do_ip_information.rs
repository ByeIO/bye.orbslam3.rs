use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DoIpInformation {
    pub physical_address: u16,
    pub functional_address: u16,
    pub target_address: u16,
}

impl Default for DoIpInformation {
    fn default() -> Self {
        DoIpInformation {
            physical_address: 0,
            functional_address: 0,
            target_address: 0,
        }
    }
}

impl ros2_client::Message for DoIpInformation {}
