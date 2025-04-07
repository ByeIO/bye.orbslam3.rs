use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RadarPreHeader {
    pub uiversionno: u16,
    pub radarpreheaderdeviceblock: crate::sick_scan_xd::msg::RadarPreHeaderDeviceBlock,
    pub radarpreheaderstatusblock: crate::sick_scan_xd::msg::RadarPreHeaderStatusBlock,
    pub radarpreheadermeasurementparam1block: crate::sick_scan_xd::msg::RadarPreHeaderMeasurementParam1Block,
    pub radarpreheaderarrayencoderblock: Vec<crate::sick_scan_xd::msg::RadarPreHeaderEncoderBlock>,
}

impl Default for RadarPreHeader {
    fn default() -> Self {
        RadarPreHeader {
            uiversionno: 0,
            radarpreheaderdeviceblock: crate::sick_scan_xd::msg::RadarPreHeaderDeviceBlock::default(),
            radarpreheaderstatusblock: crate::sick_scan_xd::msg::RadarPreHeaderStatusBlock::default(),
            radarpreheadermeasurementparam1block: crate::sick_scan_xd::msg::RadarPreHeaderMeasurementParam1Block::default(),
            radarpreheaderarrayencoderblock: Vec::new(),
        }
    }
}

impl ros2_client::Message for RadarPreHeader {}
