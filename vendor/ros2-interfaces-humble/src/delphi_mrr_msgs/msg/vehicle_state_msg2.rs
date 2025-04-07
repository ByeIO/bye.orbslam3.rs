use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VehicleStateMsg2 {
    pub header: crate::std_msgs::msg::Header,
    pub fsm_yaw_rate_valid: bool,
    pub fsm_yaw_rate: f32,
    pub can_vehicle_index_4fa: u16,
    pub fsm_vehicle_velocity: f32,
    pub can_steering_whl_angle_qf: bool,
    pub fsm_vehicle_velocity_valid: bool,
    pub can_steering_whl_angle: f32,
}

impl Default for VehicleStateMsg2 {
    fn default() -> Self {
        VehicleStateMsg2 {
            header: crate::std_msgs::msg::Header::default(),
            fsm_yaw_rate_valid: false,
            fsm_yaw_rate: 0.0,
            can_vehicle_index_4fa: 0,
            fsm_vehicle_velocity: 0.0,
            can_steering_whl_angle_qf: false,
            fsm_vehicle_velocity_valid: false,
            can_steering_whl_angle: 0.0,
        }
    }
}

impl ros2_client::Message for VehicleStateMsg2 {}
