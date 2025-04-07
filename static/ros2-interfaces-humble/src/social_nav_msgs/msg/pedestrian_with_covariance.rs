use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PedestrianWithCovariance {
    pub pedestrian: crate::social_nav_msgs::msg::Pedestrian,
    pub covariance: [f32; 4],
}

impl Default for PedestrianWithCovariance {
    fn default() -> Self {
        PedestrianWithCovariance {
            pedestrian: crate::social_nav_msgs::msg::Pedestrian::default(),
            covariance: [0.0; 4],
        }
    }
}

impl ros2_client::Message for PedestrianWithCovariance {}
