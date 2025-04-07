use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServoStatus {
    pub code: i8,
    pub message: ::std::string::String,
}

impl ServoStatus {
    pub const INVALID: i8 = -1;
    pub const NO_WARNING: i8 = 0;
    pub const DECELERATE_FOR_APPROACHING_SINGULARITY: i8 = 1;
    pub const HALT_FOR_SINGULARITY: i8 = 2;
    pub const DECELERATE_FOR_LEAVING_SINGULARITY: i8 = 3;
    pub const DECELERATE_FOR_COLLISION: i8 = 4;
    pub const HALT_FOR_COLLISION: i8 = 5;
    pub const JOINT_BOUND: i8 = 6;
}

impl Default for ServoStatus {
    fn default() -> Self {
        ServoStatus {
            code: 0,
            message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ServoStatus {}
