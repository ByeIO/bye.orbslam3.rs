use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChronyTrackingStatus {
    pub header: crate::std_msgs::msg::Header,
    pub reference: ::std::string::String,
    pub stratum: i32,
    pub system_time_offset: f64,
    pub last_offset: f64,
    pub rms_offset: f64,
    pub frequency_offset: f32,
    pub residual_frequency: f32,
    pub skew: f32,
    pub root_delay: f64,
    pub root_dispersion: f64,
    pub update_interval: f32,
    pub leap_status: ::std::string::String,
}

impl Default for ChronyTrackingStatus {
    fn default() -> Self {
        ChronyTrackingStatus {
            header: crate::std_msgs::msg::Header::default(),
            reference: ::std::string::String::new(),
            stratum: 0,
            system_time_offset: 0.0,
            last_offset: 0.0,
            rms_offset: 0.0,
            frequency_offset: 0.0,
            residual_frequency: 0.0,
            skew: 0.0,
            root_delay: 0.0,
            root_dispersion: 0.0,
            update_interval: 0.0,
            leap_status: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ChronyTrackingStatus {}
