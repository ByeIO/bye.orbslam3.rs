use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SoftwareRevisionRpt {
    pub header: crate::std_msgs::msg::Header,
    pub software_version_0: u8,
    pub software_version_1: u8,
    pub software_version_2: u8,
    pub software_day: u8,
    pub software_month_year: u16,
}

impl Default for SoftwareRevisionRpt {
    fn default() -> Self {
        SoftwareRevisionRpt {
            header: crate::std_msgs::msg::Header::default(),
            software_version_0: 0,
            software_version_1: 0,
            software_version_2: 0,
            software_day: 0,
            software_month_year: 0,
        }
    }
}

impl ros2_client::Message for SoftwareRevisionRpt {}
