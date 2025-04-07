use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Schedule {
    pub name: ::std::string::String,
    pub created_at: i64,
    pub schedule: ::std::string::String,
    pub start_at: i64,
    pub finish_at: i64, // default: 9223372036854775807
    pub group: ::std::string::String, // default: "default"
    pub payload: crate::rmf_scheduler_msgs::msg::Payload,
}

impl Default for Schedule {
    fn default() -> Self {
        Schedule {
            name: ::std::string::String::new(),
            created_at: 0,
            schedule: ::std::string::String::new(),
            start_at: 0,
            finish_at: 9223372036854775807,
            group: ::std::string::String::from("default"),
            payload: crate::rmf_scheduler_msgs::msg::Payload::default(),
        }
    }
}

impl ros2_client::Message for Schedule {}
