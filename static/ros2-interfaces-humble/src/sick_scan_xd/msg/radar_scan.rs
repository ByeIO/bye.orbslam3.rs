use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RadarScan {
    pub header: crate::std_msgs::msg::Header,
    pub radarpreheader: crate::sick_scan_xd::msg::RadarPreHeader,
    pub targets: crate::sensor_msgs::msg::PointCloud2,
    pub objects: Vec<crate::sick_scan_xd::msg::RadarObject>,
}

impl Default for RadarScan {
    fn default() -> Self {
        RadarScan {
            header: crate::std_msgs::msg::Header::default(),
            radarpreheader: crate::sick_scan_xd::msg::RadarPreHeader::default(),
            targets: crate::sensor_msgs::msg::PointCloud2::default(),
            objects: Vec::new(),
        }
    }
}

impl ros2_client::Message for RadarScan {}
