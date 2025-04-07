use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FixedFoe {
    pub header: crate::std_msgs::msg::Header,
    pub fixed_yaw: f64,
    pub fixed_horizon: f64,
}

impl Default for FixedFoe {
    fn default() -> Self {
        FixedFoe {
            header: crate::std_msgs::msg::Header::default(),
            fixed_yaw: 0.0,
            fixed_horizon: 0.0,
        }
    }
}

impl ros2_client::Message for FixedFoe {}
