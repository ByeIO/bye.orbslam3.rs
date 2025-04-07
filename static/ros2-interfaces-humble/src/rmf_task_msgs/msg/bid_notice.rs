use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BidNotice {
    pub request: ::std::string::String,
    pub task_id: ::std::string::String,
    pub time_window: crate::builtin_interfaces::msg::Duration,
}

impl Default for BidNotice {
    fn default() -> Self {
        BidNotice {
            request: ::std::string::String::new(),
            task_id: ::std::string::String::new(),
            time_window: crate::builtin_interfaces::msg::Duration::default(),
        }
    }
}

impl ros2_client::Message for BidNotice {}
