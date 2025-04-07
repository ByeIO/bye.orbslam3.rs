use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VehicleStateMsg3 {
    pub header: crate::std_msgs::msg::Header,
    pub yaw_rate_reference_valid: bool,
    pub yaw_rate_reference: f32,
    pub can_veh_long_accel_qf: u8,
    pub can_veh_long_accel: f32,
}

impl Default for VehicleStateMsg3 {
    fn default() -> Self {
        VehicleStateMsg3 {
            header: crate::std_msgs::msg::Header::default(),
            yaw_rate_reference_valid: false,
            yaw_rate_reference: 0.0,
            can_veh_long_accel_qf: 0,
            can_veh_long_accel: 0.0,
        }
    }
}

impl ros2_client::Message for VehicleStateMsg3 {}
