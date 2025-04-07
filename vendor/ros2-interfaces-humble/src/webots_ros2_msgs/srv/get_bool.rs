use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetBoolRequest {
    pub ask: bool,
}

impl Default for GetBoolRequest {
    fn default() -> Self {
        GetBoolRequest {
            ask: false,
        }
    }
}

impl ros2_client::Message for GetBoolRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetBoolResponse {
    pub value: bool,
}

impl Default for GetBoolResponse {
    fn default() -> Self {
        GetBoolResponse {
            value: false,
        }
    }
}

impl ros2_client::Message for GetBoolResponse {}


pub struct GetBool;
impl ros2_client::Service for GetBool {
    type Request = GetBoolRequest;
    type Response = GetBoolResponse;

    fn request_type_name(&self) -> &str { "GetBoolRequest" }
    fn response_type_name(&self) -> &str { "GetBoolResponse" }
}
