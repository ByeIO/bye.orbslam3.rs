use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntegerRange {
    pub from_value: i64,
    pub to_value: i64,
    pub step: u64,
}

impl Default for IntegerRange {
    fn default() -> Self {
        IntegerRange {
            from_value: 0,
            to_value: 0,
            step: 0,
        }
    }
}

impl ros2_client::Message for IntegerRange {}
