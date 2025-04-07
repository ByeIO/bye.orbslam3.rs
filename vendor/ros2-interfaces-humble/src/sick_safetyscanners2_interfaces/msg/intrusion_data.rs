use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntrusionData {
    pub data: Vec<crate::sick_safetyscanners2_interfaces::msg::IntrusionDatum>,
}

impl Default for IntrusionData {
    fn default() -> Self {
        IntrusionData {
            data: Vec::new(),
        }
    }
}

impl ros2_client::Message for IntrusionData {}
