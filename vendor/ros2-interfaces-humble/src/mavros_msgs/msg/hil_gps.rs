use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HilGPS {
    pub header: crate::std_msgs::msg::Header,
    pub fix_type: u8,
    pub geo: crate::geographic_msgs::msg::GeoPoint,
    pub eph: u16,
    pub epv: u16,
    pub vel: u16,
    pub vn: i16,
    pub ve: i16,
    pub vd: i16,
    pub cog: u16,
    pub satellites_visible: u8,
}

impl Default for HilGPS {
    fn default() -> Self {
        HilGPS {
            header: crate::std_msgs::msg::Header::default(),
            fix_type: 0,
            geo: crate::geographic_msgs::msg::GeoPoint::default(),
            eph: 0,
            epv: 0,
            vel: 0,
            vn: 0,
            ve: 0,
            vd: 0,
            cog: 0,
            satellites_visible: 0,
        }
    }
}

impl ros2_client::Message for HilGPS {}
