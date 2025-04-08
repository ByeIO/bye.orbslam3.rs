use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpawnRequest {
    pub x: f32,
    pub y: f32,
    pub theta: f32,
    pub name: ::std::string::String,
}

impl Default for SpawnRequest {
    fn default() -> Self {
        SpawnRequest {
            x: 0.0,
            y: 0.0,
            theta: 0.0,
            name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SpawnRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpawnResponse {
    pub name: ::std::string::String,
}

impl Default for SpawnResponse {
    fn default() -> Self {
        SpawnResponse {
            name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SpawnResponse {}


pub struct Spawn;
impl ros2_client::Service for Spawn {
    type Request = SpawnRequest;
    type Response = SpawnResponse;

    fn request_type_name(&self) -> &str { "SpawnRequest" }
    fn response_type_name(&self) -> &str { "SpawnResponse" }
}
