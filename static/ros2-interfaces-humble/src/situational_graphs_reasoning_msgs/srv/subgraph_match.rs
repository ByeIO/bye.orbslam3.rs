use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubgraphMatchRequest {
    pub base_graph: ::std::string::String,
    pub target_graph: ::std::string::String,
}

impl Default for SubgraphMatchRequest {
    fn default() -> Self {
        SubgraphMatchRequest {
            base_graph: ::std::string::String::new(),
            target_graph: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SubgraphMatchRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubgraphMatchResponse {
    pub success: i32,
    pub matches: Vec<crate::situational_graphs_reasoning_msgs::msg::Graph>,
    pub score: Vec<f32>,
}

impl Default for SubgraphMatchResponse {
    fn default() -> Self {
        SubgraphMatchResponse {
            success: 0,
            matches: Vec::new(),
            score: Vec::new(),
        }
    }
}

impl ros2_client::Message for SubgraphMatchResponse {}


pub struct SubgraphMatch;
impl ros2_client::Service for SubgraphMatch {
    type Request = SubgraphMatchRequest;
    type Response = SubgraphMatchResponse;

    fn request_type_name(&self) -> &str { "SubgraphMatchRequest" }
    fn response_type_name(&self) -> &str { "SubgraphMatchResponse" }
}
