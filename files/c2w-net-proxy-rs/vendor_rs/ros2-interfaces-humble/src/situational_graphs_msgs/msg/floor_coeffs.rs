use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FloorCoeffs {
    pub header: crate::std_msgs::msg::Header,
    pub coeffs: Vec<f32>,
}

impl Default for FloorCoeffs {
    fn default() -> Self {
        FloorCoeffs {
            header: crate::std_msgs::msg::Header::default(),
            coeffs: Vec::new(),
        }
    }
}

impl ros2_client::Message for FloorCoeffs {}
