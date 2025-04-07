use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JointIndexes {

}

impl JointIndexes {
    pub const HEADYAW: u8 = 0;
    pub const HEADPITCH: u8 = 1;
    pub const LSHOULDERPITCH: u8 = 2;
    pub const LSHOULDERROLL: u8 = 3;
    pub const LELBOWYAW: u8 = 4;
    pub const LELBOWROLL: u8 = 5;
    pub const LWRISTYAW: u8 = 6;
    pub const LHIPYAWPITCH: u8 = 7;
    pub const LHIPROLL: u8 = 8;
    pub const LHIPPITCH: u8 = 9;
    pub const LKNEEPITCH: u8 = 10;
    pub const LANKLEPITCH: u8 = 11;
    pub const LANKLEROLL: u8 = 12;
    pub const RHIPROLL: u8 = 13;
    pub const RHIPPITCH: u8 = 14;
    pub const RKNEEPITCH: u8 = 15;
    pub const RANKLEPITCH: u8 = 16;
    pub const RANKLEROLL: u8 = 17;
    pub const RSHOULDERPITCH: u8 = 18;
    pub const RSHOULDERROLL: u8 = 19;
    pub const RELBOWYAW: u8 = 20;
    pub const RELBOWROLL: u8 = 21;
    pub const RWRISTYAW: u8 = 22;
    pub const LHAND: u8 = 23;
    pub const RHAND: u8 = 24;
    pub const NUMJOINTS: u8 = 25;
}

impl Default for JointIndexes {
    fn default() -> Self {
        JointIndexes {

        }
    }
}

impl ros2_client::Message for JointIndexes {}
