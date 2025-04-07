use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMetadataRequest {

}

impl Default for GetMetadataRequest {
    fn default() -> Self {
        GetMetadataRequest {

        }
    }
}

impl ros2_client::Message for GetMetadataRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMetadataResponse {
    pub metadata: crate::ouster_msgs::msg::Metadata,
}

impl Default for GetMetadataResponse {
    fn default() -> Self {
        GetMetadataResponse {
            metadata: crate::ouster_msgs::msg::Metadata::default(),
        }
    }
}

impl ros2_client::Message for GetMetadataResponse {}


pub struct GetMetadata;
impl ros2_client::Service for GetMetadata {
    type Request = GetMetadataRequest;
    type Response = GetMetadataResponse;

    fn request_type_name(&self) -> &str { "GetMetadataRequest" }
    fn response_type_name(&self) -> &str { "GetMetadataResponse" }
}
