use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnsubscribeEventRequest {
    pub name: ::std::string::String,
}

impl Default for UnsubscribeEventRequest {
    fn default() -> Self {
        UnsubscribeEventRequest {
            name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for UnsubscribeEventRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnsubscribeEventResponse {

}

impl Default for UnsubscribeEventResponse {
    fn default() -> Self {
        UnsubscribeEventResponse {

        }
    }
}

impl ros2_client::Message for UnsubscribeEventResponse {}


pub struct UnsubscribeEvent;
impl ros2_client::Service for UnsubscribeEvent {
    type Request = UnsubscribeEventRequest;
    type Response = UnsubscribeEventResponse;

    fn request_type_name(&self) -> &str { "UnsubscribeEventRequest" }
    fn response_type_name(&self) -> &str { "UnsubscribeEventResponse" }
}
