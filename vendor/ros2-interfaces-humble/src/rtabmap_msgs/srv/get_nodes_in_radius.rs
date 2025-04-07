use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetNodesInRadiusRequest {
    pub node_id: i32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub radius: f32,
    pub k: i32,
}

impl Default for GetNodesInRadiusRequest {
    fn default() -> Self {
        GetNodesInRadiusRequest {
            node_id: 0,
            x: 0.0,
            y: 0.0,
            z: 0.0,
            radius: 0.0,
            k: 0,
        }
    }
}

impl ros2_client::Message for GetNodesInRadiusRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetNodesInRadiusResponse {
    pub ids: Vec<i32>,
    pub poses: Vec<crate::geometry_msgs::msg::Pose>,
    pub dists_sqr: Vec<f32>,
}

impl Default for GetNodesInRadiusResponse {
    fn default() -> Self {
        GetNodesInRadiusResponse {
            ids: Vec::new(),
            poses: Vec::new(),
            dists_sqr: Vec::new(),
        }
    }
}

impl ros2_client::Message for GetNodesInRadiusResponse {}


pub struct GetNodesInRadius;
impl ros2_client::Service for GetNodesInRadius {
    type Request = GetNodesInRadiusRequest;
    type Response = GetNodesInRadiusResponse;

    fn request_type_name(&self) -> &str { "GetNodesInRadiusRequest" }
    fn response_type_name(&self) -> &str { "GetNodesInRadiusResponse" }
}
