use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GearReport {
    pub header: crate::std_msgs::msg::Header,
    pub gear: crate::ds_dbw_msgs::msg::Gear,
    pub cmd: crate::ds_dbw_msgs::msg::Gear,
    pub driver: crate::ds_dbw_msgs::msg::Gear,
    pub reject: crate::ds_dbw_msgs::msg::GearReject,
    pub power_latched: bool,
    pub external_control: bool,
    pub ready: bool,
    pub override_active: bool,
    pub override_other: bool,
    pub fault: bool,
    pub bad_crc: bool,
    pub degraded: bool,
    pub cmd_src: crate::ds_dbw_msgs::msg::CmdSrc,
}

impl Default for GearReport {
    fn default() -> Self {
        GearReport {
            header: crate::std_msgs::msg::Header::default(),
            gear: crate::ds_dbw_msgs::msg::Gear::default(),
            cmd: crate::ds_dbw_msgs::msg::Gear::default(),
            driver: crate::ds_dbw_msgs::msg::Gear::default(),
            reject: crate::ds_dbw_msgs::msg::GearReject::default(),
            power_latched: false,
            external_control: false,
            ready: false,
            override_active: false,
            override_other: false,
            fault: false,
            bad_crc: false,
            degraded: false,
            cmd_src: crate::ds_dbw_msgs::msg::CmdSrc::default(),
        }
    }
}

impl ros2_client::Message for GearReport {}
