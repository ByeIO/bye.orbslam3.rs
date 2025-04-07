use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetROSVersionRequest {

}

impl Default for GetROSVersionRequest {
    fn default() -> Self {
        GetROSVersionRequest {

        }
    }
}

impl ros2_client::Message for GetROSVersionRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetROSVersionResponse {
    pub version: u8,
    pub distro: ::std::string::String,
}

impl Default for GetROSVersionResponse {
    fn default() -> Self {
        GetROSVersionResponse {
            version: 0,
            distro: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetROSVersionResponse {}


pub struct GetROSVersion;
impl ros2_client::Service for GetROSVersion {
    type Request = GetROSVersionRequest;
    type Response = GetROSVersionResponse;

    fn request_type_name(&self) -> &str { "GetROSVersionRequest" }
    fn response_type_name(&self) -> &str { "GetROSVersionResponse" }
}
