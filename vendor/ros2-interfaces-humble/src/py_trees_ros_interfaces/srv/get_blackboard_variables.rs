use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetBlackboardVariablesRequest {

}

impl Default for GetBlackboardVariablesRequest {
    fn default() -> Self {
        GetBlackboardVariablesRequest {

        }
    }
}

impl ros2_client::Message for GetBlackboardVariablesRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetBlackboardVariablesResponse {
    pub variables: Vec<::std::string::String>,
}

impl Default for GetBlackboardVariablesResponse {
    fn default() -> Self {
        GetBlackboardVariablesResponse {
            variables: Vec::new(),
        }
    }
}

impl ros2_client::Message for GetBlackboardVariablesResponse {}


pub struct GetBlackboardVariables;
impl ros2_client::Service for GetBlackboardVariables {
    type Request = GetBlackboardVariablesRequest;
    type Response = GetBlackboardVariablesResponse;

    fn request_type_name(&self) -> &str { "GetBlackboardVariablesRequest" }
    fn response_type_name(&self) -> &str { "GetBlackboardVariablesResponse" }
}
