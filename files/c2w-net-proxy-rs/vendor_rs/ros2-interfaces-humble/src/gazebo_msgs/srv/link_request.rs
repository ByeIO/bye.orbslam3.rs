use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LinkRequestRequest {
    pub link_name: ::std::string::String,
}

impl Default for LinkRequestRequest {
    fn default() -> Self {
        LinkRequestRequest {
            link_name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for LinkRequestRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LinkRequestResponse {

}

impl Default for LinkRequestResponse {
    fn default() -> Self {
        LinkRequestResponse {

        }
    }
}

impl ros2_client::Message for LinkRequestResponse {}


pub struct LinkRequest;
impl ros2_client::Service for LinkRequest {
    type Request = LinkRequestRequest;
    type Response = LinkRequestResponse;

    fn request_type_name(&self) -> &str { "LinkRequestRequest" }
    fn response_type_name(&self) -> &str { "LinkRequestResponse" }
}
