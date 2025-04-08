use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestNestedServiceRequest {
    pub pose: crate::geometry_msgs::msg::Pose,
}

impl Default for TestNestedServiceRequest {
    fn default() -> Self {
        TestNestedServiceRequest {
            pose: crate::geometry_msgs::msg::Pose::default(),
        }
    }
}

impl ros2_client::Message for TestNestedServiceRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestNestedServiceResponse {
    pub data: crate::std_msgs::msg::Float64,
}

impl Default for TestNestedServiceResponse {
    fn default() -> Self {
        TestNestedServiceResponse {
            data: crate::std_msgs::msg::Float64::default(),
        }
    }
}

impl ros2_client::Message for TestNestedServiceResponse {}


pub struct TestNestedService;
impl ros2_client::Service for TestNestedService {
    type Request = TestNestedServiceRequest;
    type Response = TestNestedServiceResponse;

    fn request_type_name(&self) -> &str { "TestNestedServiceRequest" }
    fn response_type_name(&self) -> &str { "TestNestedServiceResponse" }
}
