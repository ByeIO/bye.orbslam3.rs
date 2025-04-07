use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FrameGraphRequest {

}

impl Default for FrameGraphRequest {
    fn default() -> Self {
        FrameGraphRequest {

        }
    }
}

impl ros2_client::Message for FrameGraphRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FrameGraphResponse {
    pub frame_yaml: ::std::string::String,
}

impl Default for FrameGraphResponse {
    fn default() -> Self {
        FrameGraphResponse {
            frame_yaml: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for FrameGraphResponse {}


pub struct FrameGraph;
impl ros2_client::Service for FrameGraph {
    type Request = FrameGraphRequest;
    type Response = FrameGraphResponse;

    fn request_type_name(&self) -> &str { "FrameGraphRequest" }
    fn response_type_name(&self) -> &str { "FrameGraphResponse" }
}
