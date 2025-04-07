use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EsrStatus1 {
    pub header: crate::std_msgs::msg::Header,
    pub canmsg: ::std::string::String,
    pub rolling_count_1: u8,
    pub dsp_timestamp: u8,
    pub comm_error: bool,
    pub radius_curvature_calc: i16,
    pub scan_index: u16,
    pub yaw_rate_calc: f32,
    pub vehicle_speed_calc: f32,
}

impl Default for EsrStatus1 {
    fn default() -> Self {
        EsrStatus1 {
            header: crate::std_msgs::msg::Header::default(),
            canmsg: ::std::string::String::new(),
            rolling_count_1: 0,
            dsp_timestamp: 0,
            comm_error: false,
            radius_curvature_calc: 0,
            scan_index: 0,
            yaw_rate_calc: 0.0,
            vehicle_speed_calc: 0.0,
        }
    }
}

impl ros2_client::Message for EsrStatus1 {}
