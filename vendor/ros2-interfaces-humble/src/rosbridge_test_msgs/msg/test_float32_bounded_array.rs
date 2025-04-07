use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestFloat32BoundedArray {
    pub data: [f32; 16],
}

impl Default for TestFloat32BoundedArray {
    fn default() -> Self {
        TestFloat32BoundedArray {
            data: [0.0; 16],
        }
    }
}

impl ros2_client::Message for TestFloat32BoundedArray {}
