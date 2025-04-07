use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Range {
    pub header: crate::std_msgs::msg::Header,
    pub radiation_type: u8,
    pub field_of_view: f32,
    pub min_range: f32,
    pub max_range: f32,
    pub range: f32,
}

impl Range {
    pub const ULTRASOUND: u8 = 0;
    pub const INFRARED: u8 = 1;
}

impl Default for Range {
    fn default() -> Self {
        Range {
            header: crate::std_msgs::msg::Header::default(),
            radiation_type: 0,
            field_of_view: 0.0,
            min_range: 0.0,
            max_range: 0.0,
            range: 0.0,
        }
    }
}

impl ros2_client::Message for Range {}
