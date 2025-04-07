use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub header: crate::std_msgs::msg::Header,
    pub real_time_factor: f64,
    pub sensors: Vec<crate::gazebo_msgs::msg::SensorPerformanceMetric>,
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        PerformanceMetrics {
            header: crate::std_msgs::msg::Header::default(),
            real_time_factor: 0.0,
            sensors: Vec::new(),
        }
    }
}

impl ros2_client::Message for PerformanceMetrics {}
