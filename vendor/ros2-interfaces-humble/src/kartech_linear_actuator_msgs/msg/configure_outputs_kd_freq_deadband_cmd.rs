use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConfigureOutputsKdFreqDeadbandCmd {
    pub header: crate::std_msgs::msg::Header,
    pub confirm: bool,
    pub kd: u16,
    pub closed_loop_freq: u8,
    pub error_dead_band: f64,
}

impl Default for ConfigureOutputsKdFreqDeadbandCmd {
    fn default() -> Self {
        ConfigureOutputsKdFreqDeadbandCmd {
            header: crate::std_msgs::msg::Header::default(),
            confirm: false,
            kd: 0,
            closed_loop_freq: 0,
            error_dead_band: 0.0,
        }
    }
}

impl ros2_client::Message for ConfigureOutputsKdFreqDeadbandCmd {}
