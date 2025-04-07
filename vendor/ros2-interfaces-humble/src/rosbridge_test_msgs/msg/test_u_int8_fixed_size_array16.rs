use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestUInt8FixedSizeArray16 {
    pub data: [u8; 16],
}

impl Default for TestUInt8FixedSizeArray16 {
    fn default() -> Self {
        TestUInt8FixedSizeArray16 {
            data: [0; 16],
        }
    }
}

impl ros2_client::Message for TestUInt8FixedSizeArray16 {}
