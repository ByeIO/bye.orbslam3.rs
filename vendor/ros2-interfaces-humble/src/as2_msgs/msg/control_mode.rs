use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ControlMode {
    pub header: crate::std_msgs::msg::Header,
    pub yaw_mode: i8,
    pub control_mode: i8,
    pub reference_frame: i8,
}

impl ControlMode {
    pub const NONE: i8 = 0;
    pub const YAW_ANGLE: i8 = 1;
    pub const YAW_SPEED: i8 = 2;
    pub const UNSET: i8 = 0;
    pub const HOVER: i8 = 1;
    pub const POSITION: i8 = 2;
    pub const SPEED: i8 = 3;
    pub const SPEED_IN_A_PLANE: i8 = 4;
    pub const ATTITUDE: i8 = 5;
    pub const ACRO: i8 = 6;
    pub const TRAJECTORY: i8 = 7;
    pub const UNDEFINED_FRAME: i8 = 0;
    pub const LOCAL_ENU_FRAME: i8 = 1;
    pub const BODY_FLU_FRAME: i8 = 2;
    pub const GLOBAL_LAT_LONG_ASML: i8 = 3;
}

impl Default for ControlMode {
    fn default() -> Self {
        ControlMode {
            header: crate::std_msgs::msg::Header::default(),
            yaw_mode: 0,
            control_mode: 0,
            reference_frame: 0,
        }
    }
}

impl ros2_client::Message for ControlMode {}
