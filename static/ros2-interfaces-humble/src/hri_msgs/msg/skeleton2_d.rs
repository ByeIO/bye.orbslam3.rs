use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Skeleton2D {
    pub header: crate::std_msgs::msg::Header,
    pub skeleton: [crate::hri_msgs::msg::NormalizedPointOfInterest2D; 18],
}

impl Skeleton2D {
    pub const NOSE: u8 = 0;
    pub const NECK: u8 = 1;
    pub const RIGHT_SHOULDER: u8 = 2;
    pub const RIGHT_ELBOW: u8 = 3;
    pub const RIGHT_WRIST: u8 = 4;
    pub const LEFT_SHOULDER: u8 = 5;
    pub const LEFT_ELBOW: u8 = 6;
    pub const LEFT_WRIST: u8 = 7;
    pub const RIGHT_HIP: u8 = 8;
    pub const RIGHT_KNEE: u8 = 9;
    pub const RIGHT_ANKLE: u8 = 10;
    pub const LEFT_HIP: u8 = 11;
    pub const LEFT_KNEE: u8 = 12;
    pub const LEFT_ANKLE: u8 = 13;
    pub const LEFT_EYE: u8 = 14;
    pub const RIGHT_EYE: u8 = 15;
    pub const LEFT_EAR: u8 = 16;
    pub const RIGHT_EAR: u8 = 17;
}

impl Default for Skeleton2D {
    fn default() -> Self {
        Skeleton2D {
            header: crate::std_msgs::msg::Header::default(),
            skeleton: core::array::from_fn(|_| crate::hri_msgs::msg::NormalizedPointOfInterest2D::default()),
        }
    }
}

impl ros2_client::Message for Skeleton2D {}
