use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Trigger {
    pub name: ::std::string::String,
    pub created_at: i64,
    pub at: i64,
    pub group: ::std::string::String, // default: "default"
    pub payload: crate::rmf_scheduler_msgs::msg::Payload,
}

impl Default for Trigger {
    fn default() -> Self {
        Trigger {
            name: ::std::string::String::new(),
            created_at: 0,
            at: 0,
            group: ::std::string::String::from("default"),
            payload: crate::rmf_scheduler_msgs::msg::Payload::default(),
        }
    }
}

impl ros2_client::Message for Trigger {}
