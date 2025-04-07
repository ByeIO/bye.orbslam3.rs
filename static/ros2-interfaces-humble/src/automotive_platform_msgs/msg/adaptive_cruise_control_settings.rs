use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdaptiveCruiseControlSettings {
    pub header: crate::std_msgs::msg::Header,
    pub set_speed: f32,
    pub following_spot: u16,
    pub min_percent: f32,
    pub step_percent: f32,
    pub cipv_percent: f32,
    pub max_distance: f32,
}

impl Default for AdaptiveCruiseControlSettings {
    fn default() -> Self {
        AdaptiveCruiseControlSettings {
            header: crate::std_msgs::msg::Header::default(),
            set_speed: 0.0,
            following_spot: 0,
            min_percent: 0.0,
            step_percent: 0.0,
            cipv_percent: 0.0,
            max_distance: 0.0,
        }
    }
}

impl ros2_client::Message for AdaptiveCruiseControlSettings {}
