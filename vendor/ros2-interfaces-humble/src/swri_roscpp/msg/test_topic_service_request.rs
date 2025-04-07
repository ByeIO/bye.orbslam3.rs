use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestTopicServiceRequest {
    pub srv_header: crate::marti_common_msgs::msg::ServiceHeader,
    pub request_value: i32,
}

impl Default for TestTopicServiceRequest {
    fn default() -> Self {
        TestTopicServiceRequest {
            srv_header: crate::marti_common_msgs::msg::ServiceHeader::default(),
            request_value: 0,
        }
    }
}

impl ros2_client::Message for TestTopicServiceRequest {}
