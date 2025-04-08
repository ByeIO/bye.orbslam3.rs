use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DecelSrc {
    pub value: u8,
}

impl DecelSrc {
    pub const NONE: u8 = 0;
    pub const AEB: u8 = 1;
    pub const ACC: u8 = 2;
}

impl Default for DecelSrc {
    fn default() -> Self {
        DecelSrc {
            value: 0,
        }
    }
}

impl ros2_client::Message for DecelSrc {}
