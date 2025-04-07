use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SigLogEvent {
    pub time_elapsed: u32,
    pub detection_type: u8,
    pub event_type: u8,
}

impl SigLogEvent {
    pub const DETECTION_SIMULATED_SIGNAL: u8 = 0;
    pub const DETECTION_ABNORMAL_SIGNAL: u8 = 1;
    pub const DETECTION_INS_GNSS_MISMATCH: u8 = 2;
    pub const DETECTION_ABRUPT_CHANGES: u8 = 3;
    pub const DETECTION_BROADBAND_JAMMING: u8 = 4;
    pub const DETECTION_NARROWBAND_JAMMING: u8 = 5;
    pub const EVENT_STARTED: u8 = 0;
    pub const EVENT_STOPPED: u8 = 1;
    pub const EVENT_TRIGGERED: u8 = 2;
    pub const EVENT_TIMED_OUT: u8 = 3;
}

impl Default for SigLogEvent {
    fn default() -> Self {
        SigLogEvent {
            time_elapsed: 0,
            detection_type: 0,
            event_type: 0,
        }
    }
}

impl ros2_client::Message for SigLogEvent {}
