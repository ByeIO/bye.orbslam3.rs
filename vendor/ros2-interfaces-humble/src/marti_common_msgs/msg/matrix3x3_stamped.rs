use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Matrix3x3Stamped {
    pub header: crate::std_msgs::msg::Header,
    pub matrix: [f64; 9],
}

impl Default for Matrix3x3Stamped {
    fn default() -> Self {
        Matrix3x3Stamped {
            header: crate::std_msgs::msg::Header::default(),
            matrix: [0.0; 9],
        }
    }
}

impl ros2_client::Message for Matrix3x3Stamped {}
