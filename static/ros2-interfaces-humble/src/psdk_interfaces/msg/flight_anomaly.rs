use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FlightAnomaly {
    pub header: crate::std_msgs::msg::Header,
    pub impact_in_air: u32,
    pub random_fly: u32,
    pub height_ctrl_fail: u32,
    pub roll_pitch_ctrl_fail: u32,
    pub yaw_ctrl_fail: u32,
    pub aircraft_is_falling: u32,
    pub strong_wind_level1: u32,
    pub strong_wind_level2: u32,
    pub compass_installation_error: u32,
    pub imu_installation_error: u32,
    pub esc_temperature_high: u32,
    pub at_least_one_esc_disconnected: u32,
    pub gps_yaw_error: u32,
    pub reserved: u32,
}

impl Default for FlightAnomaly {
    fn default() -> Self {
        FlightAnomaly {
            header: crate::std_msgs::msg::Header::default(),
            impact_in_air: 0,
            random_fly: 0,
            height_ctrl_fail: 0,
            roll_pitch_ctrl_fail: 0,
            yaw_ctrl_fail: 0,
            aircraft_is_falling: 0,
            strong_wind_level1: 0,
            strong_wind_level2: 0,
            compass_installation_error: 0,
            imu_installation_error: 0,
            esc_temperature_high: 0,
            at_least_one_esc_disconnected: 0,
            gps_yaw_error: 0,
            reserved: 0,
        }
    }
}

impl ros2_client::Message for FlightAnomaly {}
