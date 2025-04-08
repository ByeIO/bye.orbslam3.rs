use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Session {
    pub request_time: crate::builtin_interfaces::msg::Time,
    pub requester_id: ::std::string::String,
}

impl Default for Session {
    fn default() -> Self {
        Session {
            request_time: crate::builtin_interfaces::msg::Time::default(),
            requester_id: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for Session {}
