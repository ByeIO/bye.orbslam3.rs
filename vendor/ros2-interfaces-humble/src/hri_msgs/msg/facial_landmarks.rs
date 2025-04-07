use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[serde_as]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FacialLandmarks {
    pub header: crate::std_msgs::msg::Header,
    #[serde_as(as = "[_; 70]")]
    pub landmarks: [crate::hri_msgs::msg::NormalizedPointOfInterest2D; 70],
    pub height: u32,
    pub width: u32,
}

impl FacialLandmarks {
    pub const RIGHT_EAR: u8 = 0;
    pub const RIGHT_PROFILE_1: u8 = 1;
    pub const RIGHT_PROFILE_2: u8 = 2;
    pub const RIGHT_PROFILE_3: u8 = 3;
    pub const RIGHT_PROFILE_4: u8 = 4;
    pub const RIGHT_PROFILE_5: u8 = 5;
    pub const RIGHT_PROFILE_6: u8 = 6;
    pub const RIGHT_PROFILE_7: u8 = 7;
    pub const MENTON: u8 = 8;
    pub const LEFT_EAR: u8 = 16;
    pub const LEFT_PROFILE_1: u8 = 15;
    pub const LEFT_PROFILE_2: u8 = 14;
    pub const LEFT_PROFILE_3: u8 = 13;
    pub const LEFT_PROFILE_4: u8 = 12;
    pub const LEFT_PROFILE_5: u8 = 11;
    pub const LEFT_PROFILE_6: u8 = 10;
    pub const LEFT_PROFILE_7: u8 = 9;
    pub const RIGHT_EYEBROW_OUTSIDE: u8 = 17;
    pub const RIGHT_EYEBROW_1: u8 = 18;
    pub const RIGHT_EYEBROW_2: u8 = 19;
    pub const RIGHT_EYEBROW_3: u8 = 20;
    pub const RIGHT_EYEBROW_INSIDE: u8 = 21;
    pub const RIGHT_EYE_OUTSIDE: u8 = 36;
    pub const RIGHT_EYE_TOP_1: u8 = 37;
    pub const RIGHT_EYE_TOP_2: u8 = 38;
    pub const RIGHT_EYE_INSIDE: u8 = 39;
    pub const RIGHT_EYE_BOTTOM_1: u8 = 41;
    pub const RIGHT_EYE_BOTTOM_2: u8 = 40;
    pub const RIGHT_PUPIL: u8 = 68;
    pub const LEFT_EYEBROW_OUTSIDE: u8 = 26;
    pub const LEFT_EYEBROW_1: u8 = 25;
    pub const LEFT_EYEBROW_2: u8 = 24;
    pub const LEFT_EYEBROW_3: u8 = 23;
    pub const LEFT_EYEBROW_INSIDE: u8 = 22;
    pub const LEFT_EYE_OUTSIDE: u8 = 45;
    pub const LEFT_EYE_TOP_1: u8 = 44;
    pub const LEFT_EYE_TOP_2: u8 = 43;
    pub const LEFT_EYE_INSIDE: u8 = 42;
    pub const LEFT_EYE_BOTTOM_1: u8 = 46;
    pub const LEFT_EYE_BOTTOM_2: u8 = 47;
    pub const LEFT_PUPIL: u8 = 69;
    pub const SELLION: u8 = 27;
    pub const NOSE_1: u8 = 28;
    pub const NOSE_2: u8 = 29;
    pub const NOSE: u8 = 30;
    pub const NOSTRIL_1: u8 = 31;
    pub const NOSTRIL_2: u8 = 32;
    pub const NOSTRIL_3: u8 = 33;
    pub const NOSTRIL_4: u8 = 34;
    pub const NOSTRIL_5: u8 = 35;
    pub const MOUTH_OUTER_RIGHT: u8 = 48;
    pub const MOUTH_OUTER_TOP_1: u8 = 49;
    pub const MOUTH_OUTER_TOP_2: u8 = 50;
    pub const MOUTH_OUTER_TOP_3: u8 = 51;
    pub const MOUTH_OUTER_TOP_4: u8 = 52;
    pub const MOUTH_OUTER_TOP_5: u8 = 53;
    pub const MOUTH_OUTER_LEFT: u8 = 54;
    pub const MOUTH_OUTER_BOTTOM_1: u8 = 59;
    pub const MOUTH_OUTER_BOTTOM_2: u8 = 58;
    pub const MOUTH_OUTER_BOTTOM_3: u8 = 57;
    pub const MOUTH_OUTER_BOTTOM_4: u8 = 56;
    pub const MOUTH_OUTER_BOTTOM_5: u8 = 55;
    pub const MOUTH_INNER_RIGHT: u8 = 60;
    pub const MOUTH_INNER_TOP_1: u8 = 61;
    pub const MOUTH_INNER_TOP_2: u8 = 62;
    pub const MOUTH_INNER_TOP_3: u8 = 63;
    pub const MOUTH_INNER_LEFT: u8 = 64;
    pub const MOUTH_INNER_BOTTOM_1: u8 = 67;
    pub const MOUTH_INNER_BOTTOM_2: u8 = 66;
    pub const MOUTH_INNER_BOTTOM_3: u8 = 65;
}

impl Default for FacialLandmarks {
    fn default() -> Self {
        FacialLandmarks {
            header: crate::std_msgs::msg::Header::default(),
            landmarks: core::array::from_fn(|_| crate::hri_msgs::msg::NormalizedPointOfInterest2D::default()),
            height: 0,
            width: 0,
        }
    }
}

impl ros2_client::Message for FacialLandmarks {}
