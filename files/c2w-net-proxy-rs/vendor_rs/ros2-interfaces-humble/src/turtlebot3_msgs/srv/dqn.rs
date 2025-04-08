use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DqnRequest {
    pub action: u8,
    pub init: bool,
}

impl Default for DqnRequest {
    fn default() -> Self {
        DqnRequest {
            action: 0,
            init: false,
        }
    }
}

impl ros2_client::Message for DqnRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DqnResponse {
    pub state: Vec<f32>,
    pub reward: f32,
    pub done: bool,
}

impl Default for DqnResponse {
    fn default() -> Self {
        DqnResponse {
            state: Vec::new(),
            reward: 0.0,
            done: false,
        }
    }
}

impl ros2_client::Message for DqnResponse {}


pub struct Dqn;
impl ros2_client::Service for Dqn {
    type Request = DqnRequest;
    type Response = DqnResponse;

    fn request_type_name(&self) -> &str { "DqnRequest" }
    fn response_type_name(&self) -> &str { "DqnResponse" }
}
