use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpticalFlowRad {
    pub header: crate::std_msgs::msg::Header,
    pub integration_time_us: u32,
    pub integrated_x: f32,
    pub integrated_y: f32,
    pub integrated_xgyro: f32,
    pub integrated_ygyro: f32,
    pub integrated_zgyro: f32,
    pub temperature: i16,
    pub quality: u8,
    pub time_delta_distance_us: u32,
    pub distance: f32,
}

impl Default for OpticalFlowRad {
    fn default() -> Self {
        OpticalFlowRad {
            header: crate::std_msgs::msg::Header::default(),
            integration_time_us: 0,
            integrated_x: 0.0,
            integrated_y: 0.0,
            integrated_xgyro: 0.0,
            integrated_ygyro: 0.0,
            integrated_zgyro: 0.0,
            temperature: 0,
            quality: 0,
            time_delta_distance_us: 0,
            distance: 0.0,
        }
    }
}

impl ros2_client::Message for OpticalFlowRad {}
