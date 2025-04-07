use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateRigidBodyRequest {
    pub rigid_body_name: ::std::string::String,
    pub link_parent: ::std::string::String,
    pub markers: Vec<i32>,
}

impl Default for CreateRigidBodyRequest {
    fn default() -> Self {
        CreateRigidBodyRequest {
            rigid_body_name: ::std::string::String::new(),
            link_parent: ::std::string::String::new(),
            markers: Vec::new(),
        }
    }
}

impl ros2_client::Message for CreateRigidBodyRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateRigidBodyResponse {
    pub success: bool,
}

impl Default for CreateRigidBodyResponse {
    fn default() -> Self {
        CreateRigidBodyResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for CreateRigidBodyResponse {}


pub struct CreateRigidBody;
impl ros2_client::Service for CreateRigidBody {
    type Request = CreateRigidBodyRequest;
    type Response = CreateRigidBodyResponse;

    fn request_type_name(&self) -> &str { "CreateRigidBodyRequest" }
    fn response_type_name(&self) -> &str { "CreateRigidBodyResponse" }
}
