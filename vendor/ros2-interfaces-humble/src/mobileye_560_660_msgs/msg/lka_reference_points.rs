use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LkaReferencePoints {
    pub header: crate::std_msgs::msg::Header,
    pub ref_point_1_position: f64,
    pub ref_point_1_distance: f64,
    pub ref_point_1_validity: bool,
    pub ref_point_2_position: f64,
    pub ref_point_2_distance: f64,
    pub ref_point_2_validity: bool,
}

impl Default for LkaReferencePoints {
    fn default() -> Self {
        LkaReferencePoints {
            header: crate::std_msgs::msg::Header::default(),
            ref_point_1_position: 0.0,
            ref_point_1_distance: 0.0,
            ref_point_1_validity: false,
            ref_point_2_position: 0.0,
            ref_point_2_distance: 0.0,
            ref_point_2_validity: false,
        }
    }
}

impl ros2_client::Message for LkaReferencePoints {}
