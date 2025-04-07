use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TurnSignalReport {
    pub header: crate::std_msgs::msg::Header,
    pub input: crate::ds_dbw_msgs::msg::TurnSignal,
    pub cmd: crate::ds_dbw_msgs::msg::TurnSignal,
    pub output: crate::ds_dbw_msgs::msg::TurnSignal,
    pub feedback: crate::ds_dbw_msgs::msg::TurnSignal,
    pub ready: bool,
    pub override_active: bool,
    pub override_other: bool,
    pub timeout: bool,
    pub bad_crc: bool,
    pub bad_rc: bool,
    pub degraded: bool,
    pub degraded_cmd_type: bool,
    pub degraded_comms_dbw_steer: bool,
    pub degraded_comms_dbw_brake: bool,
    pub degraded_comms_dbw_thrtl: bool,
    pub degraded_comms_vehicle: bool,
    pub degraded_control_performance: bool,
    pub fault: bool,
    pub fault_comms_vehicle: bool,
}

impl Default for TurnSignalReport {
    fn default() -> Self {
        TurnSignalReport {
            header: crate::std_msgs::msg::Header::default(),
            input: crate::ds_dbw_msgs::msg::TurnSignal::default(),
            cmd: crate::ds_dbw_msgs::msg::TurnSignal::default(),
            output: crate::ds_dbw_msgs::msg::TurnSignal::default(),
            feedback: crate::ds_dbw_msgs::msg::TurnSignal::default(),
            ready: false,
            override_active: false,
            override_other: false,
            timeout: false,
            bad_crc: false,
            bad_rc: false,
            degraded: false,
            degraded_cmd_type: false,
            degraded_comms_dbw_steer: false,
            degraded_comms_dbw_brake: false,
            degraded_comms_dbw_thrtl: false,
            degraded_comms_vehicle: false,
            degraded_control_performance: false,
            fault: false,
            fault_comms_vehicle: false,
        }
    }
}

impl ros2_client::Message for TurnSignalReport {}
