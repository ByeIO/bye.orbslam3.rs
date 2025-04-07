use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EyeTracker {
    pub header: crate::std_msgs::msg::Header,
    pub attention: f32,
    pub gaze: u8,
    pub eyes_present: bool,
}

impl EyeTracker {
    pub const GAZE_OTHER: u8 = 0;
    pub const GAZE_PASSENGER_WINDSHIELD: u8 = 1;
    pub const GAZE_DRIVER_WINDSHIELD: u8 = 2;
    pub const GAZE_INSTRUMENT_PANEL: u8 = 3;
    pub const GAZE_MEDIA_TABLET: u8 = 4;
    pub const GAZE_REAR_MIRROR: u8 = 6;
    pub const GAZE_DRIVER_MIRROR: u8 = 8;
    pub const GAZE_PASSENGER_MIRROR: u8 = 9;
    pub const GAZE_BELOW_DASHBOARD: u8 = 10;
}

impl Default for EyeTracker {
    fn default() -> Self {
        EyeTracker {
            header: crate::std_msgs::msg::Header::default(),
            attention: 0.0,
            gaze: 0,
            eyes_present: false,
        }
    }
}

impl ros2_client::Message for EyeTracker {}
