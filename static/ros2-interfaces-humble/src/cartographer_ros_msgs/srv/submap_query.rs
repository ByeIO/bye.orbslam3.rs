use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubmapQueryRequest {
    pub trajectory_id: i32,
    pub submap_index: i32,
}

impl Default for SubmapQueryRequest {
    fn default() -> Self {
        SubmapQueryRequest {
            trajectory_id: 0,
            submap_index: 0,
        }
    }
}

impl ros2_client::Message for SubmapQueryRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubmapQueryResponse {
    pub status: crate::cartographer_ros_msgs::msg::StatusResponse,
    pub submap_version: i32,
    pub textures: Vec<crate::cartographer_ros_msgs::msg::SubmapTexture>,
}

impl Default for SubmapQueryResponse {
    fn default() -> Self {
        SubmapQueryResponse {
            status: crate::cartographer_ros_msgs::msg::StatusResponse::default(),
            submap_version: 0,
            textures: Vec::new(),
        }
    }
}

impl ros2_client::Message for SubmapQueryResponse {}


pub struct SubmapQuery;
impl ros2_client::Service for SubmapQuery {
    type Request = SubmapQueryRequest;
    type Response = SubmapQueryResponse;

    fn request_type_name(&self) -> &str { "SubmapQueryRequest" }
    fn response_type_name(&self) -> &str { "SubmapQueryResponse" }
}
