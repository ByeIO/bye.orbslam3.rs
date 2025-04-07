use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldLineArray {
    pub lines: Vec<crate::soccer_object_msgs::msg::FieldLine>,
}

impl Default for FieldLineArray {
    fn default() -> Self {
        FieldLineArray {
            lines: Vec::new(),
        }
    }
}

impl ros2_client::Message for FieldLineArray {}
