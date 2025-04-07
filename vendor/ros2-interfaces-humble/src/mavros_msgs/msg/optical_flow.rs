use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpticalFlow {
    pub header: crate::std_msgs::msg::Header,
    pub flow: crate::geometry_msgs::msg::Vector3,
    pub flow_comp_m: crate::geometry_msgs::msg::Vector3,
    pub quality: u8,
    pub ground_distance: f32,
    pub flow_rate: crate::geometry_msgs::msg::Vector3,
}

impl Default for OpticalFlow {
    fn default() -> Self {
        OpticalFlow {
            header: crate::std_msgs::msg::Header::default(),
            flow: crate::geometry_msgs::msg::Vector3::default(),
            flow_comp_m: crate::geometry_msgs::msg::Vector3::default(),
            quality: 0,
            ground_distance: 0.0,
            flow_rate: crate::geometry_msgs::msg::Vector3::default(),
        }
    }
}

impl ros2_client::Message for OpticalFlow {}
