use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SteerMode {
    pub header: crate::std_msgs::msg::Header,
    pub mode: u16,
    pub curvature: f32,
    pub max_curvature_rate: f32,
}

impl Default for SteerMode {
    fn default() -> Self {
        SteerMode {
            header: crate::std_msgs::msg::Header::default(),
            mode: 0,
            curvature: 0.0,
            max_curvature_rate: 0.0,
        }
    }
}

impl ros2_client::Message for SteerMode {}
