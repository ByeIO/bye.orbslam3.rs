use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SensorPerformanceMetric {
    pub name: ::std::string::String,
    pub sim_update_rate: f64,
    pub real_update_rate: f64,
    pub fps: f64,
}

impl Default for SensorPerformanceMetric {
    fn default() -> Self {
        SensorPerformanceMetric {
            name: ::std::string::String::new(),
            sim_update_rate: 0.0,
            real_update_rate: 0.0,
            fps: 0.0,
        }
    }
}

impl ros2_client::Message for SensorPerformanceMetric {}
