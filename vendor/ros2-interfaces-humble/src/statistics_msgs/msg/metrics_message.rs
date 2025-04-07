use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetricsMessage {
    pub measurement_source_name: ::std::string::String,
    pub metrics_source: ::std::string::String,
    pub unit: ::std::string::String,
    pub window_start: crate::builtin_interfaces::msg::Time,
    pub window_stop: crate::builtin_interfaces::msg::Time,
    pub statistics: Vec<crate::statistics_msgs::msg::StatisticDataPoint>,
}

impl Default for MetricsMessage {
    fn default() -> Self {
        MetricsMessage {
            measurement_source_name: ::std::string::String::new(),
            metrics_source: ::std::string::String::new(),
            unit: ::std::string::String::new(),
            window_start: crate::builtin_interfaces::msg::Time::default(),
            window_stop: crate::builtin_interfaces::msg::Time::default(),
            statistics: Vec::new(),
        }
    }
}

impl ros2_client::Message for MetricsMessage {}
