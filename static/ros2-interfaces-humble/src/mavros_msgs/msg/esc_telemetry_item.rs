use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ESCTelemetryItem {
    pub header: crate::std_msgs::msg::Header,
    pub temperature: f32,
    pub voltage: f32,
    pub current: f32,
    pub totalcurrent: f32,
    pub rpm: i32,
    pub count: u16,
}

impl Default for ESCTelemetryItem {
    fn default() -> Self {
        ESCTelemetryItem {
            header: crate::std_msgs::msg::Header::default(),
            temperature: 0.0,
            voltage: 0.0,
            current: 0.0,
            totalcurrent: 0.0,
            rpm: 0,
            count: 0,
        }
    }
}

impl ros2_client::Message for ESCTelemetryItem {}
