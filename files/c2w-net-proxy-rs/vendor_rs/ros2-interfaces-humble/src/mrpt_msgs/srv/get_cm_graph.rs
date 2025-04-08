use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCMGraphRequest {
    pub node_ids: Vec<u64>,
}

impl Default for GetCMGraphRequest {
    fn default() -> Self {
        GetCMGraphRequest {
            node_ids: Vec::new(),
        }
    }
}

impl ros2_client::Message for GetCMGraphRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCMGraphResponse {
    pub cm_graph: crate::mrpt_msgs::msg::NetworkOfPoses,
}

impl Default for GetCMGraphResponse {
    fn default() -> Self {
        GetCMGraphResponse {
            cm_graph: crate::mrpt_msgs::msg::NetworkOfPoses::default(),
        }
    }
}

impl ros2_client::Message for GetCMGraphResponse {}


pub struct GetCMGraph;
impl ros2_client::Service for GetCMGraph {
    type Request = GetCMGraphRequest;
    type Response = GetCMGraphResponse;

    fn request_type_name(&self) -> &str { "GetCMGraphRequest" }
    fn response_type_name(&self) -> &str { "GetCMGraphResponse" }
}
