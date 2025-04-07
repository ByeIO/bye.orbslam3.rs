use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ColaMsgSrvRequest {
    pub request: ::std::string::String,
}

impl Default for ColaMsgSrvRequest {
    fn default() -> Self {
        ColaMsgSrvRequest {
            request: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ColaMsgSrvRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ColaMsgSrvResponse {
    pub response: ::std::string::String,
}

impl Default for ColaMsgSrvResponse {
    fn default() -> Self {
        ColaMsgSrvResponse {
            response: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ColaMsgSrvResponse {}


pub struct ColaMsgSrv;
impl ros2_client::Service for ColaMsgSrv {
    type Request = ColaMsgSrvRequest;
    type Response = ColaMsgSrvResponse;

    fn request_type_name(&self) -> &str { "ColaMsgSrvRequest" }
    fn response_type_name(&self) -> &str { "ColaMsgSrvResponse" }
}
