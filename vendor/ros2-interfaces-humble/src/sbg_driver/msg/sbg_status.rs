use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SbgStatus {
    pub header: crate::std_msgs::msg::Header,
    pub time_stamp: u32,
    pub status_general: crate::sbg_driver::msg::SbgStatusGeneral,
    pub status_com: crate::sbg_driver::msg::SbgStatusCom,
    pub status_aiding: crate::sbg_driver::msg::SbgStatusAiding,
}

impl Default for SbgStatus {
    fn default() -> Self {
        SbgStatus {
            header: crate::std_msgs::msg::Header::default(),
            time_stamp: 0,
            status_general: crate::sbg_driver::msg::SbgStatusGeneral::default(),
            status_com: crate::sbg_driver::msg::SbgStatusCom::default(),
            status_aiding: crate::sbg_driver::msg::SbgStatusAiding::default(),
        }
    }
}

impl ros2_client::Message for SbgStatus {}
