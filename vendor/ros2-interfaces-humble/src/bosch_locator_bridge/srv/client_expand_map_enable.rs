use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClientExpandMapEnableRequest {
    pub prior_map_name: ::std::string::String,
}

impl Default for ClientExpandMapEnableRequest {
    fn default() -> Self {
        ClientExpandMapEnableRequest {
            prior_map_name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ClientExpandMapEnableRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClientExpandMapEnableResponse {

}

impl Default for ClientExpandMapEnableResponse {
    fn default() -> Self {
        ClientExpandMapEnableResponse {

        }
    }
}

impl ros2_client::Message for ClientExpandMapEnableResponse {}


pub struct ClientExpandMapEnable;
impl ros2_client::Service for ClientExpandMapEnable {
    type Request = ClientExpandMapEnableRequest;
    type Response = ClientExpandMapEnableResponse;

    fn request_type_name(&self) -> &str { "ClientExpandMapEnableRequest" }
    fn response_type_name(&self) -> &str { "ClientExpandMapEnableResponse" }
}
