use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EsrStatus2 {
    pub header: crate::std_msgs::msg::Header,
    pub canmsg: ::std::string::String,
    pub maximum_tracks_ack: u8,
    pub rolling_count_2: u8,
    pub overheat_error: bool,
    pub range_perf_error: bool,
    pub internal_error: bool,
    pub xcvr_operational: bool,
    pub raw_data_mode: bool,
    pub steering_angle_ack: u16,
    pub temperature: i8,
    pub veh_spd_comp_factor: f32,
    pub grouping_mode: u8,
    pub yaw_rate_bias: f32,
    pub sw_version_dsp: ::std::string::String,
}

impl Default for EsrStatus2 {
    fn default() -> Self {
        EsrStatus2 {
            header: crate::std_msgs::msg::Header::default(),
            canmsg: ::std::string::String::new(),
            maximum_tracks_ack: 0,
            rolling_count_2: 0,
            overheat_error: false,
            range_perf_error: false,
            internal_error: false,
            xcvr_operational: false,
            raw_data_mode: false,
            steering_angle_ack: 0,
            temperature: 0,
            veh_spd_comp_factor: 0.0,
            grouping_mode: 0,
            yaw_rate_bias: 0.0,
            sw_version_dsp: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for EsrStatus2 {}
