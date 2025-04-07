use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Int8 {
    pub data: i8,
}

impl Default for Int8 {
    fn default() -> Self {
        Int8 {
            data: 0,
        }
    }
}

impl ros2_client::Message for Int8 {}
