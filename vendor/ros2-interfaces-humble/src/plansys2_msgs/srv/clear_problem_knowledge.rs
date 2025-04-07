use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClearProblemKnowledgeRequest {
    pub request: crate::std_msgs::msg::Empty,
}

impl Default for ClearProblemKnowledgeRequest {
    fn default() -> Self {
        ClearProblemKnowledgeRequest {
            request: crate::std_msgs::msg::Empty::default(),
        }
    }
}

impl ros2_client::Message for ClearProblemKnowledgeRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClearProblemKnowledgeResponse {
    pub success: bool,
    pub error_info: ::std::string::String,
}

impl Default for ClearProblemKnowledgeResponse {
    fn default() -> Self {
        ClearProblemKnowledgeResponse {
            success: false,
            error_info: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ClearProblemKnowledgeResponse {}


pub struct ClearProblemKnowledge;
impl ros2_client::Service for ClearProblemKnowledge {
    type Request = ClearProblemKnowledgeRequest;
    type Response = ClearProblemKnowledgeResponse;

    fn request_type_name(&self) -> &str { "ClearProblemKnowledgeRequest" }
    fn response_type_name(&self) -> &str { "ClearProblemKnowledgeResponse" }
}
