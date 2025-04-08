use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntrusionDatum {
    pub size: u32,
    pub flags: Vec<bool>,
}

impl Default for IntrusionDatum {
    fn default() -> Self {
        IntrusionDatum {
            size: 0,
            flags: Vec::new(),
        }
    }
}

impl ros2_client::Message for IntrusionDatum {}
