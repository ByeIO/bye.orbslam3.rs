use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteEntityRequest {
    pub entity: crate::ros_gz_interfaces::msg::Entity,
}

impl Default for DeleteEntityRequest {
    fn default() -> Self {
        DeleteEntityRequest {
            entity: crate::ros_gz_interfaces::msg::Entity::default(),
        }
    }
}

impl ros2_client::Message for DeleteEntityRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteEntityResponse {
    pub success: bool,
}

impl Default for DeleteEntityResponse {
    fn default() -> Self {
        DeleteEntityResponse {
            success: false,
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
