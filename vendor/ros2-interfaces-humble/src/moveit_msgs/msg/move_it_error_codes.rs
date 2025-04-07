use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MoveItErrorCodes {
    pub val: i32,
    pub message: ::std::string::String,
    pub source: ::std::string::String,
}

impl MoveItErrorCodes {
    pub const SUCCESS: i32 = 1;
    pub const UNDEFINED: i32 = 0;
    pub const FAILURE: i32 = 99999;
    pub const PLANNING_FAILED: i32 = -1;
    pub const INVALID_MOTION_PLAN: i32 = -2;
    pub const MOTION_PLAN_INVALIDATED_BY_ENVIRONMENT_CHANGE: i32 = -3;
    pub const CONTROL_FAILED: i32 = -4;
    pub const UNABLE_TO_AQUIRE_SENSOR_DATA: i32 = -5;
    pub const TIMED_OUT: i32 = -6;
    pub const PREEMPTED: i32 = -7;
    pub const START_STATE_IN_COLLISION: i32 = -10;
    pub const START_STATE_VIOLATES_PATH_CONSTRAINTS: i32 = -11;
    pub const START_STATE_INVALID: i32 = -26;
    pub const GOAL_IN_COLLISION: i32 = -12;
    pub const GOAL_VIOLATES_PATH_CONSTRAINTS: i32 = -13;
    pub const GOAL_CONSTRAINTS_VIOLATED: i32 = -14;
    pub const GOAL_STATE_INVALID: i32 = -27;
    pub const UNRECOGNIZED_GOAL_TYPE: i32 = -28;
    pub const INVALID_GROUP_NAME: i32 = -15;
    pub const INVALID_GOAL_CONSTRAINTS: i32 = -16;
    pub const INVALID_ROBOT_STATE: i32 = -17;
    pub const INVALID_LINK_NAME: i32 = -18;
    pub const INVALID_OBJECT_NAME: i32 = -19;
    pub const FRAME_TRANSFORM_FAILURE: i32 = -21;
    pub const COLLISION_CHECKING_UNAVAILABLE: i32 = -22;
    pub const ROBOT_STATE_STALE: i32 = -23;
    pub const SENSOR_INFO_STALE: i32 = -24;
    pub const COMMUNICATION_FAILURE: i32 = -25;
    pub const CRASH: i32 = -29;
    pub const ABORT: i32 = -30;
    pub const NO_IK_SOLUTION: i32 = -31;
}

impl Default for MoveItErrorCodes {
    fn default() -> Self {
        MoveItErrorCodes {
            val: 0,
            message: ::std::string::String::new(),
            source: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for MoveItErrorCodes {}
