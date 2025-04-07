use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScanData2204 {
    pub header: crate::std_msgs::msg::Header,
    pub ibeo_header: crate::ibeo_msgs::msg::IbeoDataHeader,
    pub scan_start_time: u64,
    pub scan_end_time_offset: u32,
    pub ground_labeled: bool,
    pub dirt_labeled: bool,
    pub rain_labeled: bool,
    pub fused_scan: bool,
    pub mirror_side: u8,
    pub coordinate_system: u8,
    pub scan_number: u16,
    pub scan_points: u16,
    pub number_of_scanner_infos: u16,
    pub scanner_info_list: Vec<crate::ibeo_msgs::msg::ScannerInfo2204>,
    pub scan_point_list: Vec<crate::ibeo_msgs::msg::ScanPoint2204>,
}

impl ScanData2204 {
    pub const FRONT: u8 = 0;
    pub const REAR: u8 = 1;
    pub const SCANNER: u8 = 0;
    pub const VEHICLE: u8 = 1;
}

impl Default for ScanData2204 {
    fn default() -> Self {
        ScanData2204 {
            header: crate::std_msgs::msg::Header::default(),
            ibeo_header: crate::ibeo_msgs::msg::IbeoDataHeader::default(),
            scan_start_time: 0,
            scan_end_time_offset: 0,
            ground_labeled: false,
            dirt_labeled: false,
            rain_labeled: false,
            fused_scan: false,
            mirror_side: 0,
            coordinate_system: 0,
            scan_number: 0,
            scan_points: 0,
            number_of_scanner_infos: 0,
            scanner_info_list: Vec::new(),
            scan_point_list: Vec::new(),
        }
    }
}

impl ros2_client::Message for ScanData2204 {}
