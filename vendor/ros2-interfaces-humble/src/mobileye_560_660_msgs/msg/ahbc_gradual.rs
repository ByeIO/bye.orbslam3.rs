use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AhbcGradual {
    pub header: crate::std_msgs::msg::Header,
    pub boundary_domain_bottom_non_glare_hlb: f32,
    pub boundary_domain_non_glare_left_hand_hlb: f32,
    pub boundary_domain_non_glare_right_hand_hlb: f32,
    pub object_distance_hlb: u16,
    pub status_boundary_domain_bottom_non_glare_hlb: u8,
    pub status_boundary_domain_non_glare_left_hand_hlb: u8,
    pub status_boundary_domain_non_glare_right_hand_hlb: u8,
    pub status_object_distance_hlb: u8,
    pub left_target_change: bool,
    pub right_target_change: bool,
    pub too_many_cars: bool,
    pub busy_scene: bool,
}

impl Default for AhbcGradual {
    fn default() -> Self {
        AhbcGradual {
            header: crate::std_msgs::msg::Header::default(),
            boundary_domain_bottom_non_glare_hlb: 0.0,
            boundary_domain_non_glare_left_hand_hlb: 0.0,
            boundary_domain_non_glare_right_hand_hlb: 0.0,
            object_distance_hlb: 0,
            status_boundary_domain_bottom_non_glare_hlb: 0,
            status_boundary_domain_non_glare_left_hand_hlb: 0,
            status_boundary_domain_non_glare_right_hand_hlb: 0,
            status_object_distance_hlb: 0,
            left_target_change: false,
            right_target_change: false,
            too_many_cars: false,
            busy_scene: false,
        }
    }
}

impl ros2_client::Message for AhbcGradual {}
