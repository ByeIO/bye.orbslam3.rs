use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Altimeter {
    pub header: crate::std_msgs::msg::Header,
    pub vertical_position: f64,
    pub vertical_velocity: f64,
    pub vertical_reference: f64,
}

impl Default for Altimeter {
    fn default() -> Self {
        Altimeter {
            header: crate::std_msgs::msg::Header::default(),
            vertical_position: 0.0,
            vertical_velocity: 0.0,
            vertical_reference: 0.0,
        }
    }
}

impl ros2_client::Message for Altimeter {}
