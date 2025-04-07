use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NavControllerOutput {
    pub header: crate::std_msgs::msg::Header,
    pub nav_roll: f32,
    pub nav_pitch: f32,
    pub nav_bearing: i16,
    pub target_bearing: i16,
    pub wp_dist: u16,
    pub alt_error: f32,
    pub aspd_error: f32,
    pub xtrack_error: f32,
}

impl Default for NavControllerOutput {
    fn default() -> Self {
        NavControllerOutput {
            header: crate::std_msgs::msg::Header::default(),
            nav_roll: 0.0,
            nav_pitch: 0.0,
            nav_bearing: 0,
            target_bearing: 0,
            wp_dist: 0,
            alt_error: 0.0,
            aspd_error: 0.0,
            xtrack_error: 0.0,
        }
    }
}

impl ros2_client::Message for NavControllerOutput {}
