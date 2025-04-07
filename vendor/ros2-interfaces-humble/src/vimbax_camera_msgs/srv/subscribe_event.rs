use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubscribeEventRequest {
    pub name: ::std::string::String,
}

impl Default for SubscribeEventRequest {
    fn default() -> Self {
        SubscribeEventRequest {
            name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SubscribeEventRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubscribeEventResponse {
    pub error: crate::vimbax_camera_msgs::msg::Error,
}

impl Default for SubscribeEventResponse {
    fn default() -> Self {
        SubscribeEventResponse {
            error: crate::vimbax_camera_msgs::msg::Error::default(),
        }
    }
}

impl ros2_client::Message for SubscribeEventResponse {}


pub struct SubscribeEvent;
impl ros2_client::Service for SubscribeEvent {
    type Request = SubscribeEventRequest;
    type Response = SubscribeEventResponse;

    fn request_type_name(&self) -> &str { "SubscribeEventRequest" }
    fn response_type_name(&self) -> &str { "SubscribeEventResponse" }
}
