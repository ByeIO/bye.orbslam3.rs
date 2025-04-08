use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestUInt8 {
    pub data: Vec<u8>,
}

impl Default for TestUInt8 {
    fn default() -> Self {
        TestUInt8 {
            data: Vec::new(),
        }
    }
}

impl ros2_client::Message for TestUInt8 {}
