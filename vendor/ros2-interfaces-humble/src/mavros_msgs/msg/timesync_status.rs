use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimesyncStatus {
    pub header: crate::std_msgs::msg::Header,
    pub remote_timestamp_ns: u64,
    pub observed_offset_ns: i64,
    pub estimated_offset_ns: i64,
    pub round_trip_time_ms: f32,
}

impl Default for TimesyncStatus {
    fn default() -> Self {
        TimesyncStatus {
            header: crate::std_msgs::msg::Header::default(),
            remote_timestamp_ns: 0,
            observed_offset_ns: 0,
            estimated_offset_ns: 0,
            round_trip_time_ms: 0.0,
        }
    }
}

impl ros2_client::Message for TimesyncStatus {}
