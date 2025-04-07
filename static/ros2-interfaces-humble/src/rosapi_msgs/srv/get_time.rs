use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetTimeRequest {

}

impl Default for GetTimeRequest {
    fn default() -> Self {
        GetTimeRequest {

        }
    }
}

impl ros2_client::Message for GetTimeRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetTimeResponse {
    pub time: crate::builtin_interfaces::msg::Time,
}

impl Default for GetTimeResponse {
    fn default() -> Self {
        GetTimeResponse {
            time: crate::builtin_interfaces::msg::Time::default(),
        }
    }
}

impl ros2_client::Message for GetTimeResponse {}


pub struct GetTime;
impl ros2_client::Service for GetTime {
    type Request = GetTimeRequest;
    type Response = GetTimeResponse;

    fn request_type_name(&self) -> &str { "GetTimeRequest" }
    fn response_type_name(&self) -> &str { "GetTimeResponse" }
}
