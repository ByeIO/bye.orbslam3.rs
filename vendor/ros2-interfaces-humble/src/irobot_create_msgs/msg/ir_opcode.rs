use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IrOpcode {
    pub header: crate::std_msgs::msg::Header,
    pub opcode: u8,
    pub sensor: u8,
}

impl IrOpcode {
    pub const CODE_IR_FORCE_FIELD: u8 = 161;
    pub const CODE_IR_VIRTUAL_WALL: u8 = 162;
    pub const CODE_IR_BUOY_GREEN: u8 = 164;
    pub const CODE_IR_BUOY_RED: u8 = 168;
    pub const CODE_IR_BUOY_BOTH: u8 = 172;
    pub const CODE_IR_EVAC_GREEN_FIELD: u8 = 244;
    pub const CODE_IR_EVAC_RED_FIELD: u8 = 248;
    pub const CODE_IR_EVAC_BOTH_FIELD: u8 = 252;
    pub const SENSOR_OMNI: u8 = 0;
    pub const SENSOR_DIRECTIONAL_FRONT: u8 = 1;
}

impl Default for IrOpcode {
    fn default() -> Self {
        IrOpcode {
            header: crate::std_msgs::msg::Header::default(),
            opcode: 0,
            sensor: 0,
        }
    }
}

impl ros2_client::Message for IrOpcode {}
