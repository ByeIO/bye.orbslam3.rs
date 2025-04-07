use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestHeaderTwo {
    pub header: crate::std_msgs::msg::Header,
}

impl Default for TestHeaderTwo {
    fn default() -> Self {
        TestHeaderTwo {
            header: crate::std_msgs::msg::Header::default(),
        }
    }
}

impl ros2_client::Message for TestHeaderTwo {}
