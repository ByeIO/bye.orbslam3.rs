use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HealthStatus {
    pub header: crate::std_msgs::msg::Header,
    pub status: i8,
    pub message: ::std::string::String,
}

impl HealthStatus {
    pub const OK: i8 = 0;
    pub const WARN: i8 = 1;
    pub const ERROR: i8 = 2;
    pub const STALE: i8 = 3;
}

impl Default for HealthStatus {
    fn default() -> Self {
        HealthStatus {
            header: crate::std_msgs::msg::Header::default(),
            status: 0,
            message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for HealthStatus {}
