use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetStateRequest {
    pub time_stamp: crate::builtin_interfaces::msg::Time,
    pub frame_id: ::std::string::String,
}

impl Default for GetStateRequest {
    fn default() -> Self {
        GetStateRequest {
            time_stamp: crate::builtin_interfaces::msg::Time::default(),
            frame_id: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetStateRequest {}


use serde_with::serde_as;

#[serde_as]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetStateResponse {
    pub state: [f64; 15],
    #[serde_as(as = "[_; 225]")]
    pub covariance: [f64; 225],
}

impl Default for GetStateResponse {
    fn default() -> Self {
        GetStateResponse {
            state: [0.0; 15],
            covariance: [0.0; 225],
        }
    }
}

impl ros2_client::Message for GetStateResponse {}


pub struct GetState;
impl ros2_client::Service for GetState {
    type Request = GetStateRequest;
    type Response = GetStateResponse;

    fn request_type_name(&self) -> &str { "GetStateRequest" }
    fn response_type_name(&self) -> &str { "GetStateResponse" }
}
