use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntrospectSubscribersRequest {

}

impl Default for IntrospectSubscribersRequest {
    fn default() -> Self {
        IntrospectSubscribersRequest {

        }
    }
}

impl ros2_client::Message for IntrospectSubscribersRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntrospectSubscribersResponse {
    pub subscriber_details: Vec<crate::py_trees_ros_interfaces::msg::SubscriberDetails>,
}

impl Default for IntrospectSubscribersResponse {
    fn default() -> Self {
        IntrospectSubscribersResponse {
            subscriber_details: Vec::new(),
        }
    }
}

impl ros2_client::Message for IntrospectSubscribersResponse {}


pub struct IntrospectSubscribers;
impl ros2_client::Service for IntrospectSubscribers {
    type Request = IntrospectSubscribersRequest;
    type Response = IntrospectSubscribersResponse;

    fn request_type_name(&self) -> &str { "IntrospectSubscribersRequest" }
    fn response_type_name(&self) -> &str { "IntrospectSubscribersResponse" }
}
