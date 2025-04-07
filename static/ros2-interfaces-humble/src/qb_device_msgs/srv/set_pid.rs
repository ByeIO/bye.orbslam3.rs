use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetPidRequest {
    pub id: i32,
    pub max_repeats: i32,
    pub p: f32,
    pub i: f32,
    pub d: f32,
}

impl Default for SetPidRequest {
    fn default() -> Self {
        SetPidRequest {
            id: 0,
            max_repeats: 0,
            p: 0.0,
            i: 0.0,
            d: 0.0,
        }
    }
}

impl ros2_client::Message for SetPidRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetPidResponse {
    pub success: bool,
    pub failures: i32,
}

impl Default for SetPidResponse {
    fn default() -> Self {
        SetPidResponse {
            success: false,
            failures: 0,
        }
    }
}

impl ros2_client::Message for SetPidResponse {}


pub struct SetPid;
impl ros2_client::Service for SetPid {
    type Request = SetPidRequest;
    type Response = SetPidResponse;

    fn request_type_name(&self) -> &str { "SetPidRequest" }
    fn response_type_name(&self) -> &str { "SetPidResponse" }
}
