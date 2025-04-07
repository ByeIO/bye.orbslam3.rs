use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Inspector {
    pub stamp: crate::builtin_interfaces::msg::Time,
    pub cluster_name: ::std::string::String,
    pub cluster_size: u32,
    pub id: u32,
    pub state: u8,
    pub term: u64,
    pub data_size: u64,
    pub voted_for: u32,
}

impl Inspector {
    pub const UNKNOWN: u8 = 0;
    pub const STANDBY: u8 = 1;
    pub const FOLLOWER: u8 = 2;
    pub const CANDIDATE: u8 = 3;
    pub const LEADER: u8 = 4;
    pub const TOPIC_NAME: &'static str = "foros/inspector";
}

impl Default for Inspector {
    fn default() -> Self {
        Inspector {
            stamp: crate::builtin_interfaces::msg::Time::default(),
            cluster_name: ::std::string::String::new(),
            cluster_size: 0,
            id: 0,
            state: 0,
            term: 0,
            data_size: 0,
            voted_for: 0,
        }
    }
}

impl ros2_client::Message for Inspector {}
