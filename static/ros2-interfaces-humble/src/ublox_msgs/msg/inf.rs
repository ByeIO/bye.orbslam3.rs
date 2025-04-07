use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Inf {
    pub str: Vec<i8>,
}

impl Inf {
    pub const CLASS_ID: u8 = 4;
}

impl Default for Inf {
    fn default() -> Self {
        Inf {
            str: Vec::new(),
        }
    }
}

impl ros2_client::Message for Inf {}
