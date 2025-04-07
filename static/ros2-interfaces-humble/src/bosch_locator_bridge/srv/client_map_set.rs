use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClientMapSetRequest {
    pub name: ::std::string::String,
}

impl Default for ClientMapSetRequest {
    fn default() -> Self {
        ClientMapSetRequest {
            name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ClientMapSetRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClientMapSetResponse {

}

impl Default for ClientMapSetResponse {
    fn default() -> Self {
        ClientMapSetResponse {

        }
    }
}

impl ros2_client::Message for ClientMapSetResponse {}


pub struct ClientMapSet;
impl ros2_client::Service for ClientMapSet {
    type Request = ClientMapSetRequest;
    type Response = ClientMapSetResponse;

    fn request_type_name(&self) -> &str { "ClientMapSetRequest" }
    fn response_type_name(&self) -> &str { "ClientMapSetResponse" }
}
