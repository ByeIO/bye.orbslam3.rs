use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestArray {
    pub data: Vec<f64>,
}

impl Default for TestArray {
    fn default() -> Self {
        TestArray {
            data: Vec::new(),
        }
    }
}

impl ros2_client::Message for TestArray {}
