use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Statistics {
    pub count: i64,
    pub stamp: crate::builtin_interfaces::msg::Time,
    pub tick_duration: f32,
    pub tick_duration_average: f32,
    pub tick_duration_variance: f32,
    pub tick_interval: f32,
    pub tick_interval_average: f32,
    pub tick_interval_variance: f32,
}

impl Default for Statistics {
    fn default() -> Self {
        Statistics {
            count: 0,
            stamp: crate::builtin_interfaces::msg::Time::default(),
            tick_duration: 0.0,
            tick_duration_average: 0.0,
            tick_duration_variance: 0.0,
            tick_interval: 0.0,
            tick_interval_average: 0.0,
            tick_interval_variance: 0.0,
        }
    }
}

impl ros2_client::Message for Statistics {}
