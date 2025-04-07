use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SbgUtcTime {
    pub header: crate::std_msgs::msg::Header,
    pub time_stamp: u32,
    pub clock_status: crate::sbg_driver::msg::SbgUtcTimeStatus,
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub min: u8,
    pub sec: u8,
    pub nanosec: u32,
    pub gps_tow: u32,
    pub clk_bias_std: f32,
    pub clk_sf_error_std: f32,
    pub clk_residual_error: f32,
}

impl Default for SbgUtcTime {
    fn default() -> Self {
        SbgUtcTime {
            header: crate::std_msgs::msg::Header::default(),
            time_stamp: 0,
            clock_status: crate::sbg_driver::msg::SbgUtcTimeStatus::default(),
            year: 0,
            month: 0,
            day: 0,
            hour: 0,
            min: 0,
            sec: 0,
            nanosec: 0,
            gps_tow: 0,
            clk_bias_std: 0.0,
            clk_sf_error_std: 0.0,
            clk_residual_error: 0.0,
        }
    }
}

impl ros2_client::Message for SbgUtcTime {}
