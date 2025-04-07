use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DifferentialMeasurement {
    pub header: crate::std_msgs::msg::Header,
    pub base_frame_id: ::std::string::String,
    pub baseline_length: f64,
    pub baseline_length_variance: f64,
    pub heading: f64,
    pub heading_variance: f64,
    pub pitch: f64,
    pub pitch_variance: f64,
    pub roll: f64,
    pub roll_variance: f64,
    pub position: crate::geometry_msgs::msg::Vector3,
    pub position_covariance: [f64; 9],
}

impl Default for DifferentialMeasurement {
    fn default() -> Self {
        DifferentialMeasurement {
            header: crate::std_msgs::msg::Header::default(),
            base_frame_id: ::std::string::String::new(),
            baseline_length: 0.0,
            baseline_length_variance: 0.0,
            heading: 0.0,
            heading_variance: 0.0,
            pitch: 0.0,
            pitch_variance: 0.0,
            roll: 0.0,
            roll_variance: 0.0,
            position: crate::geometry_msgs::msg::Vector3::default(),
            position_covariance: [0.0; 9],
        }
    }
}

impl ros2_client::Message for DifferentialMeasurement {}
