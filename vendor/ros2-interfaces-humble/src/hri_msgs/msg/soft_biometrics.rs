use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SoftBiometrics {
    pub header: crate::std_msgs::msg::Header,
    pub age: u8,
    pub age_confidence: f32,
    pub gender: u8,
    pub gender_confidence: f32,
}

impl SoftBiometrics {
    pub const UNDEFINED: u8 = 0;
    pub const FEMALE: u8 = 1;
    pub const MALE: u8 = 2;
    pub const OTHER: u8 = 3;
}

impl Default for SoftBiometrics {
    fn default() -> Self {
        SoftBiometrics {
            header: crate::std_msgs::msg::Header::default(),
            age: 0,
            age_confidence: 0.0,
            gender: 0,
            gender_confidence: 0.0,
        }
    }
}

impl ros2_client::Message for SoftBiometrics {}
