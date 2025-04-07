use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SickLdmrsObjectArray {
    pub header: crate::sick_scan_xd::msg::Header,
    pub objects: Vec<crate::sick_scan_xd::msg::SickLdmrsObject>,
}

impl Default for SickLdmrsObjectArray {
    fn default() -> Self {
        SickLdmrsObjectArray {
            header: crate::sick_scan_xd::msg::Header::default(),
            objects: Vec::new(),
        }
    }
}

impl ros2_client::Message for SickLdmrsObjectArray {}
