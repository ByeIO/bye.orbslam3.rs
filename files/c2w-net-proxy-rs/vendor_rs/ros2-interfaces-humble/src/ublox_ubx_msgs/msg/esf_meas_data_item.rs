use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ESFMeasDataItem {
    pub data_field: u32,
    pub data_type: u8,
}

impl Default for ESFMeasDataItem {
    fn default() -> Self {
        ESFMeasDataItem {
            data_field: 0,
            data_type: 0,
        }
    }
}

impl ros2_client::Message for ESFMeasDataItem {}
