use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Lane {
    pub header: crate::std_msgs::msg::Header,
    pub lane_curvature: f64,
    pub lane_heading: f64,
    pub construction_area: bool,
    pub pitch_angle: f64,
    pub yaw_angle: f64,
    pub right_ldw_availability: bool,
    pub left_ldw_availability: bool,
}

impl Default for Lane {
    fn default() -> Self {
        Lane {
            header: crate::std_msgs::msg::Header::default(),
            lane_curvature: 0.0,
            lane_heading: 0.0,
            construction_area: false,
            pitch_angle: 0.0,
            yaw_angle: 0.0,
            right_ldw_availability: false,
            left_ldw_availability: false,
        }
    }
}

impl ros2_client::Message for Lane {}
