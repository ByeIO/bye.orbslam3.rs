use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BodyRequestRequest {
    pub body_name: ::std::string::String,
}

impl Default for BodyRequestRequest {
    fn default() -> Self {
        BodyRequestRequest {
            body_name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for BodyRequestRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BodyRequestResponse {

}

impl Default for BodyRequestResponse {
    fn default() -> Self {
        BodyRequestResponse {

        }
    }
}

impl ros2_client::Message for BodyRequestResponse {}


pub struct BodyRequest;
impl ros2_client::Service for BodyRequest {
    type Request = BodyRequestRequest;
    type Response = BodyRequestResponse;

    fn request_type_name(&self) -> &str { "BodyRequestRequest" }
    fn response_type_name(&self) -> &str { "BodyRequestResponse" }
}
