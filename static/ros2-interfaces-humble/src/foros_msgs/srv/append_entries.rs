use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppendEntriesRequest {
    pub term: u64,
    pub leader_id: u32,
    pub prev_log_index: u64,
    pub prev_log_term: u64,
    pub entries: Vec<u8>,
    pub leader_commit: u64,
}

impl Default for AppendEntriesRequest {
    fn default() -> Self {
        AppendEntriesRequest {
            term: 0,
            leader_id: 0,
            prev_log_index: 0,
            prev_log_term: 0,
            entries: Vec::new(),
            leader_commit: 0,
        }
    }
}

impl ros2_client::Message for AppendEntriesRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppendEntriesResponse {
    pub term: u64,
    pub success: bool,
}

impl Default for AppendEntriesResponse {
    fn default() -> Self {
        AppendEntriesResponse {
            term: 0,
            success: false,
        }
    }
}

impl ros2_client::Message for AppendEntriesResponse {}


pub struct AppendEntries;
impl ros2_client::Service for AppendEntries {
    type Request = AppendEntriesRequest;
    type Response = AppendEntriesResponse;

    fn request_type_name(&self) -> &str { "AppendEntriesRequest" }
    fn response_type_name(&self) -> &str { "AppendEntriesResponse" }
}
