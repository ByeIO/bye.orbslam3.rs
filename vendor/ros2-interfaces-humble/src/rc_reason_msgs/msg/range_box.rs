use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RangeBox {
    pub min_dimensions: crate::rc_reason_msgs::msg::Box,
    pub max_dimensions: crate::rc_reason_msgs::msg::Box,
}

impl Default for RangeBox {
    fn default() -> Self {
        RangeBox {
            min_dimensions: crate::rc_reason_msgs::msg::Box::default(),
            max_dimensions: crate::rc_reason_msgs::msg::Box::default(),
        }
    }
}

impl ros2_client::Message for RangeBox {}
