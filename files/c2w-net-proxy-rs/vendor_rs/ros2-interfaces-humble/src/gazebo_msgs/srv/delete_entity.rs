use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteEntityRequest {
    pub name: ::std::string::String,
}

impl Default for DeleteEntityRequest {
    fn default() -> Self {
        DeleteEntityRequest {
            name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for DeleteEntityRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteEntityResponse {
    pub success: bool,
    pub status_message: ::std::string::String,
}

impl Default for DeleteEntityResponse {
    fn default() -> Self {
        DeleteEntityResponse {
            success: false,
            status_message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for DeleteEntityResponse {}


pub struct DeleteEntity;
impl ros2_client::Service for DeleteEntity {
    type Request = DeleteEntityRequest;
    type Response = DeleteEntityResponse;

    fn request_type_name(&self) -> &str { "DeleteEntityRequest" }
    fn response_type_name(&self) -> &str { "DeleteEntityResponse" }
}
