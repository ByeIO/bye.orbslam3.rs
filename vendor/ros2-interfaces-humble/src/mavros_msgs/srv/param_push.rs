use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParamPushRequest {

}

impl Default for ParamPushRequest {
    fn default() -> Self {
        ParamPushRequest {

        }
    }
}

impl ros2_client::Message for ParamPushRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParamPushResponse {
    pub success: bool,
    pub param_transfered: u32,
}

impl Default for ParamPushResponse {
    fn default() -> Self {
        ParamPushResponse {
            success: false,
            param_transfered: 0,
        }
    }
}

impl ros2_client::Message for ParamPushResponse {}


pub struct ParamPush;
impl ros2_client::Service for ParamPush {
    type Request = ParamPushRequest;
    type Response = ParamPushResponse;

    fn request_type_name(&self) -> &str { "ParamPushRequest" }
    fn response_type_name(&self) -> &str { "ParamPushResponse" }
}
