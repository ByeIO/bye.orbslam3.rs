use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddSubmapRequest {
    pub filename: ::std::string::String,
}

impl Default for AddSubmapRequest {
    fn default() -> Self {
        AddSubmapRequest {
            filename: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for AddSubmapRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddSubmapResponse {

}

impl Default for AddSubmapResponse {
    fn default() -> Self {
        AddSubmapResponse {

        }
    }
}

impl ros2_client::Message for AddSubmapResponse {}


pub struct AddSubmap;
impl ros2_client::Service for AddSubmap {
    type Request = AddSubmapRequest;
    type Response = AddSubmapResponse;

    fn request_type_name(&self) -> &str { "AddSubmapRequest" }
    fn response_type_name(&self) -> &str { "AddSubmapResponse" }
}
