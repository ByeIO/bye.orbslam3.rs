use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestNestedBoundedArray {
    pub data: crate::rosbridge_test_msgs::msg::TestFloat32BoundedArray,
}

impl Default for TestNestedBoundedArray {
    fn default() -> Self {
        TestNestedBoundedArray {
            data: crate::rosbridge_test_msgs::msg::TestFloat32BoundedArray::default(),
        }
    }
}

impl ros2_client::Message for TestNestedBoundedArray {}
