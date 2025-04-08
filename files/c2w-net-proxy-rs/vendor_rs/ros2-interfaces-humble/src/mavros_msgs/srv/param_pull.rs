use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParamPullRequest {
    pub force_pull: bool,
}

impl Default for ParamPullRequest {
    fn default() -> Self {
        ParamPullRequest {
            force_pull: false,
        }
    }
}

impl ros2_client::Message for ParamPullRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParamPullResponse {
    pub success: bool,
    pub param_received: u32,
}

impl Default for ParamPullResponse {
    fn default() -> Self {
        ParamPullResponse {
            success: false,
            param_received: 0,
        }
    }
}

impl ros2_client::Message for ParamPullResponse {}


pub struct ParamPull;
impl ros2_client::Service for ParamPull {
    type Request = ParamPullRequest;
    type Response = ParamPullResponse;

    fn request_type_name(&self) -> &str { "ParamPullRequest" }
    fn response_type_name(&self) -> &str { "ParamPullResponse" }
}
