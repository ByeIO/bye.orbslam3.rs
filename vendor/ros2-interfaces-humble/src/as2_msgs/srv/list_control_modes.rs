use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListControlModesRequest {

}

impl Default for ListControlModesRequest {
    fn default() -> Self {
        ListControlModesRequest {

        }
    }
}

impl ros2_client::Message for ListControlModesRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListControlModesResponse {
    pub source: ::std::string::String,
    pub control_modes: Vec<u8>,
}

impl Default for ListControlModesResponse {
    fn default() -> Self {
        ListControlModesResponse {
            source: ::std::string::String::new(),
            control_modes: Vec::new(),
        }
    }
}

impl ros2_client::Message for ListControlModesResponse {}


pub struct ListControlModes;
impl ros2_client::Service for ListControlModes {
    type Request = ListControlModesRequest;
    type Response = ListControlModesResponse;

    fn request_type_name(&self) -> &str { "ListControlModesRequest" }
    fn response_type_name(&self) -> &str { "ListControlModesResponse" }
}
