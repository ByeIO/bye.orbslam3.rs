use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RequestVoteRequest {
    pub term: u64,
    pub candidate_id: u32,
    pub last_data_index: u64,
    pub loat_data_term: u64,
}

impl Default for RequestVoteRequest {
    fn default() -> Self {
        RequestVoteRequest {
            term: 0,
            candidate_id: 0,
            last_data_index: 0,
            loat_data_term: 0,
        }
    }
}

impl ros2_client::Message for RequestVoteRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RequestVoteResponse {
    pub term: u64,
    pub vote_granted: bool,
}

impl Default for RequestVoteResponse {
    fn default() -> Self {
        RequestVoteResponse {
            term: 0,
            vote_granted: false,
        }
    }
}

impl ros2_client::Message for RequestVoteResponse {}


pub struct RequestVote;
impl ros2_client::Service for RequestVote {
    type Request = RequestVoteRequest;
    type Response = RequestVoteResponse;

    fn request_type_name(&self) -> &str { "RequestVoteRequest" }
    fn response_type_name(&self) -> &str { "RequestVoteResponse" }
}
