use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HmsInfoTable {
    pub num_msg: u32,
    pub table: Vec<crate::psdk_interfaces::msg::HmsInfoMsg>,
}

impl Default for HmsInfoTable {
    fn default() -> Self {
        HmsInfoTable {
            num_msg: 0,
            table: Vec::new(),
        }
    }
}

impl ros2_client::Message for HmsInfoTable {}
