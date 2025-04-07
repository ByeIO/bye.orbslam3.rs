use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetricFamily {
    pub name: ::std::string::String,
    pub description: ::std::string::String,
    pub metrics: Vec<crate::cartographer_ros_msgs::msg::Metric>,
}

impl Default for MetricFamily {
    fn default() -> Self {
        MetricFamily {
            name: ::std::string::String::new(),
            description: ::std::string::String::new(),
            metrics: Vec::new(),
        }
    }
}

impl ros2_client::Message for MetricFamily {}
