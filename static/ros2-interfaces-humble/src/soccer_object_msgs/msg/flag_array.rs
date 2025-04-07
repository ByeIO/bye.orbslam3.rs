use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FlagArray {
    pub flags: Vec<crate::soccer_object_msgs::msg::Flag>,
}

impl Default for FlagArray {
    fn default() -> Self {
        FlagArray {
            flags: Vec::new(),
        }
    }
}

impl ros2_client::Message for FlagArray {}
