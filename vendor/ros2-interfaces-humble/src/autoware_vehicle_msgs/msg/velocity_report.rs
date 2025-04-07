use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VelocityReport {
    pub header: crate::std_msgs::msg::Header,
    pub longitudinal_velocity: f32,
    pub lateral_velocity: f32,
    pub heading_rate: f32,
}

impl Default for VelocityReport {
    fn default() -> Self {
        VelocityReport {
            header: crate::std_msgs::msg::Header::default(),
            longitudinal_velocity: 0.0,
            lateral_velocity: 0.0,
            heading_rate: 0.0,
        }
    }
}

impl ros2_client::Message for VelocityReport {}
