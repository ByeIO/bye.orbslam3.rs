use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VelocityAccelCov {
    pub header: crate::std_msgs::msg::Header,
    pub velocity: f32,
    pub accleration: f32,
    pub covariance: f32,
}

impl Default for VelocityAccelCov {
    fn default() -> Self {
        VelocityAccelCov {
            header: crate::std_msgs::msg::Header::default(),
            velocity: 0.0,
            accleration: 0.0,
            covariance: 0.0,
        }
    }
}

impl ros2_client::Message for VelocityAccelCov {}
