use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EsrVehicle5 {
    pub header: crate::std_msgs::msg::Header,
    pub oversteer_understeer: i8,
    pub yaw_rate_bias_shift: bool,
    pub beamwidth_vert: f32,
    pub funnel_offset_left: f32,
    pub funnel_offset_right: f32,
    pub cw_blockage_threshold: f32,
    pub distance_rear_axle: u16,
    pub wheelbase: u16,
    pub steering_gear_ratio: f32,
}

impl Default for EsrVehicle5 {
    fn default() -> Self {
        EsrVehicle5 {
            header: crate::std_msgs::msg::Header::default(),
            oversteer_understeer: 0,
            yaw_rate_bias_shift: false,
            beamwidth_vert: 0.0,
            funnel_offset_left: 0.0,
            funnel_offset_right: 0.0,
            cw_blockage_threshold: 0.0,
            distance_rear_axle: 0,
            wheelbase: 0,
            steering_gear_ratio: 0.0,
        }
    }
}

impl ros2_client::Message for EsrVehicle5 {}
