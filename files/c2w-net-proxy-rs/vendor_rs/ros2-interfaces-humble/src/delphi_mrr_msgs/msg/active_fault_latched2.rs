use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActiveFaultLatched2 {
    pub header: crate::std_msgs::msg::Header,
    pub ipma_pcan_data_range_check: bool,
    pub ipma_pcan_missing_msg: bool,
    pub vin_signal_compare_failure: bool,
    pub module_not_configured_error: bool,
    pub car_cfg_not_configured_error: bool,
}

impl Default for ActiveFaultLatched2 {
    fn default() -> Self {
        ActiveFaultLatched2 {
            header: crate::std_msgs::msg::Header::default(),
            ipma_pcan_data_range_check: false,
            ipma_pcan_missing_msg: false,
            vin_signal_compare_failure: false,
            module_not_configured_error: false,
            car_cfg_not_configured_error: false,
        }
    }
}

impl ros2_client::Message for ActiveFaultLatched2 {}
