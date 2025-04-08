use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UBXRxmMeasx {
    pub header: crate::std_msgs::msg::Header,
    pub version: u8,
    pub gps_tow: u32,
    pub glo_tow: u32,
    pub bds_tow: u32,
    pub qzss_tow: u32,
    pub gps_tow_acc: u16,
    pub glo_tow_acc: u16,
    pub bds_tow_acc: u16,
    pub qzss_tow_acc: u16,
    pub num_sv: u8,
    pub flags: u8,
    pub sv_data: Vec<crate::ublox_ubx_msgs::msg::MeasxData>,
}

impl UBXRxmMeasx {
    pub const TOW_NOT_SET: u8 = 0;
    pub const TOW_SET: u8 = 1;
    pub const TOW_SET2: u8 = 2;
}

impl Default for UBXRxmMeasx {
    fn default() -> Self {
        UBXRxmMeasx {
            header: crate::std_msgs::msg::Header::default(),
            version: 0,
            gps_tow: 0,
            glo_tow: 0,
            bds_tow: 0,
            qzss_tow: 0,
            gps_tow_acc: 0,
            glo_tow_acc: 0,
            bds_tow_acc: 0,
            qzss_tow_acc: 0,
            num_sv: 0,
            flags: 0,
            sv_data: Vec::new(),
        }
    }
}

impl ros2_client::Message for UBXRxmMeasx {}
