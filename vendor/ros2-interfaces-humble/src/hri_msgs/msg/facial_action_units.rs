use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[serde_as]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FacialActionUnits {
    pub header: crate::std_msgs::msg::Header,
    #[serde_as(as = "[_; 99]")]
    pub intensity: [f32; 99],
    #[serde_as(as = "[_; 99]")]
    pub confidence: [f32; 99],
}

impl FacialActionUnits {
    pub const NEUTRAL_FACE: u8 = 0;
    pub const INNER_BROW_RAISER: u8 = 1;
    pub const OUTER_BROW_RAISER: u8 = 2;
    pub const BROW_LOWERER: u8 = 4;
    pub const UPPER_LID_RAISER: u8 = 5;
    pub const CHEEK_RAISER: u8 = 6;
    pub const LID_TIGHTENER: u8 = 7;
    pub const LIPS_TOWARD_EACH_OTHER: u8 = 8;
    pub const NOSE_WRINKLER: u8 = 9;
    pub const UPPER_LIP_RAISER: u8 = 10;
    pub const NASOLABIAL_DEEPENER: u8 = 11;
    pub const LIP_CORNER_PULLER: u8 = 12;
    pub const SHARP_LIP_PULLER: u8 = 13;
    pub const DIMPLER: u8 = 14;
    pub const LIP_CORNER_DEPRESSOR: u8 = 15;
    pub const LOWER_LIP_DEPRESSOR: u8 = 16;
    pub const CHIN_RAISER: u8 = 17;
    pub const LIP_PUCKER: u8 = 18;
    pub const TONGUE_SHOW: u8 = 19;
    pub const LIP_STRETCHER: u8 = 20;
    pub const NECK_TIGHTENER: u8 = 21;
    pub const LIP_FUNNELER: u8 = 22;
    pub const LIP_TIGHTENER: u8 = 23;
    pub const LIP_PRESSOR: u8 = 24;
    pub const LIPS_PART: u8 = 25;
    pub const JAW_DROP: u8 = 26;
    pub const MOUTH_STRETCH: u8 = 27;
    pub const LIP_SUCK: u8 = 28;
    pub const HEAD_TURN_LEFT: u8 = 51;
    pub const HEAD_TURN_RIGHT: u8 = 52;
    pub const HEAD_UP: u8 = 53;
    pub const HEAD_DOWN: u8 = 54;
    pub const HEAD_TILT_LEFT: u8 = 55;
    pub const HEAD_TILT_RIGHT: u8 = 56;
    pub const HEAD_FORWARD: u8 = 57;
    pub const HEAD_BACK: u8 = 58;
    pub const EYES_TURN_LEFT: u8 = 61;
    pub const EYES_TURN_RIGHT: u8 = 62;
    pub const EYES_UP: u8 = 63;
    pub const EYES_DOWN: u8 = 64;
    pub const WALLEYE: u8 = 65;
    pub const CROSS_EYE: u8 = 66;
    pub const EYES_POSITIONED_TO_LOOK_AT_OTHER_PERSON: u8 = 69;
    pub const BROWS_AND_FOREHEAD_NOT_VISIBLE: u8 = 70;
    pub const EYES_NOT_VISIBLE: u8 = 71;
    pub const LOWER_FACE_NOT_VISIBLE: u8 = 72;
    pub const ENTIRE_FACE_NOT_VISIBLE: u8 = 73;
    pub const UNSOCIABLE: u8 = 74;
    pub const JAW_THRUST: u8 = 29;
    pub const JAW_SIDEWAYS: u8 = 30;
    pub const JAW_CLENCHER: u8 = 31;
    pub const LIP_BITE: u8 = 32;
    pub const CHEEK_BLOW: u8 = 33;
    pub const CHEEK_PUFF: u8 = 34;
    pub const CHEEK_SUCK: u8 = 35;
    pub const TONGUE_BULGE: u8 = 36;
    pub const LIP_WIPE: u8 = 37;
    pub const NOSTRIL_DILATOR: u8 = 38;
    pub const NOSTRIL_COMPRESSOR: u8 = 39;
    pub const SNIFF: u8 = 40;
    pub const LID_DROOP: u8 = 41;
    pub const SLIT: u8 = 42;
    pub const EYES_CLOSED: u8 = 43;
    pub const SQUINT: u8 = 44;
    pub const BLINK: u8 = 45;
    pub const WINK: u8 = 46;
    pub const SPEECH: u8 = 50;
    pub const SWALLOW: u8 = 80;
    pub const CHEWING: u8 = 81;
    pub const SHOULDER_SHRUG: u8 = 82;
    pub const HEAD_SHAKE_BACK_AND_FORTH: u8 = 84;
    pub const HEAD_NOD_UP_AND_DOWN: u8 = 85;
    pub const FLASH: u8 = 91;
    pub const PARTIAL_FLASH: u8 = 92;
    pub const SHIVER_TREMBLE: u8 = 97;
    pub const FAST_UP_DOWN_LOOK: u8 = 98;
}

impl Default for FacialActionUnits {
    fn default() -> Self {
        FacialActionUnits {
            header: crate::std_msgs::msg::Header::default(),
            intensity: [0.0; 99],
            confidence: [0.0; 99],
        }
    }
}

impl ros2_client::Message for FacialActionUnits {}
