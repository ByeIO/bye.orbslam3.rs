use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetIORequest {
    pub fun: i8,
    pub pin: i8,
    pub state: f32,
}

impl SetIORequest {
    pub const PIN_AOUT0: i8 = 0;
    pub const PIN_AOUT1: i8 = 1;
    pub const PIN_DOUT0: i8 = 0;
    pub const PIN_DOUT1: i8 = 1;
    pub const PIN_DOUT2: i8 = 2;
    pub const PIN_DOUT3: i8 = 3;
    pub const PIN_DOUT4: i8 = 4;
    pub const PIN_DOUT5: i8 = 5;
    pub const PIN_DOUT6: i8 = 6;
    pub const PIN_DOUT7: i8 = 7;
    pub const PIN_CONF_OUT0: i8 = 8;
    pub const PIN_CONF_OUT1: i8 = 9;
    pub const PIN_CONF_OUT2: i8 = 10;
    pub const PIN_CONF_OUT3: i8 = 11;
    pub const PIN_CONF_OUT4: i8 = 12;
    pub const PIN_CONF_OUT5: i8 = 13;
    pub const PIN_CONF_OUT6: i8 = 14;
    pub const PIN_CONF_OUT7: i8 = 15;
    pub const PIN_TOOL_DOUT0: i8 = 16;
    pub const PIN_TOOL_DOUT1: i8 = 17;
    pub const FUN_SET_DIGITAL_OUT: i8 = 1;
    pub const FUN_SET_FLAG: i8 = 2;
    pub const FUN_SET_ANALOG_OUT: i8 = 3;
    pub const FUN_SET_TOOL_VOLTAGE: i8 = 4;
    pub const STATE_OFF: i8 = 0;
    pub const STATE_ON: i8 = 1;
    pub const STATE_TOOL_VOLTAGE_0V: i8 = 0;
    pub const STATE_TOOL_VOLTAGE_12V: i8 = 12;
    pub const STATE_TOOL_VOLTAGE_24V: i8 = 24;
}

impl Default for SetIORequest {
    fn default() -> Self {
        SetIORequest {
            fun: 0,
            pin: 0,
            state: 0.0,
        }
    }
}

impl ros2_client::Message for SetIORequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetIOResponse {
    pub success: bool,
}

impl Default for SetIOResponse {
    fn default() -> Self {
        SetIOResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for SetIOResponse {}


pub struct SetIO;
impl ros2_client::Service for SetIO {
    type Request = SetIORequest;
    type Response = SetIOResponse;

    fn request_type_name(&self) -> &str { "SetIORequest" }
    fn response_type_name(&self) -> &str { "SetIOResponse" }
}
