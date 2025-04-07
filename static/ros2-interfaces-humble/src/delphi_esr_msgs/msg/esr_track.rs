use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EsrTrack {
    pub header: crate::std_msgs::msg::Header,
    pub canmsg: ::std::string::String,
    pub id: u8,
    pub lat_rate: f32,
    pub grouping_changed: bool,
    pub oncoming: bool,
    pub status: u8,
    pub angle: f32,
    pub range: f32,
    pub bridge_object: bool,
    pub rolling_count: bool,
    pub width: f32,
    pub range_accel: f32,
    pub med_range_mode: u8,
    pub range_rate: f32,
}

impl Default for EsrTrack {
    fn default() -> Self {
        EsrTrack {
            header: crate::std_msgs::msg::Header::default(),
            canmsg: ::std::string::String::new(),
            id: 0,
            lat_rate: 0.0,
            grouping_changed: false,
            oncoming: false,
            status: 0,
            angle: 0.0,
            range: 0.0,
            bridge_object: false,
            rolling_count: false,
            width: 0.0,
            range_accel: 0.0,
            med_range_mode: 0,
            range_rate: 0.0,
        }
    }
}

impl ros2_client::Message for EsrTrack {}
