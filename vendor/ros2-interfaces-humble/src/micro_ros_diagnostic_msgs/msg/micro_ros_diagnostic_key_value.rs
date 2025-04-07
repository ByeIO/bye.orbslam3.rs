use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MicroROSDiagnosticKeyValue {
    pub level: u8,
    pub key: u16,
    pub value_type: u8,
    pub bool_value: bool,
    pub int_value: i32,
    pub double_value: f32,
    pub value_id: u16,
}

impl MicroROSDiagnosticKeyValue {
    pub const OK: u8 = 0;
    pub const WARN: u8 = 1;
    pub const ERROR: u8 = 2;
    pub const STALE: u8 = 3;
    pub const VALUE_BOOL: u8 = 1;
    pub const VALUE_INT: u8 = 2;
    pub const VALUE_FLOAT: u8 = 3;
    pub const VALUE_LOOKUP: u8 = 10;
}

impl Default for MicroROSDiagnosticKeyValue {
    fn default() -> Self {
        MicroROSDiagnosticKeyValue {
            level: 0,
            key: 0,
            value_type: 0,
            bool_value: false,
            int_value: 0,
            double_value: 0.0,
            value_id: 0,
        }
    }
}

impl ros2_client::Message for MicroROSDiagnosticKeyValue {}
