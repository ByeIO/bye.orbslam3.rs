use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BurstRequest {
    pub num_messages: u64,
}

impl Default for BurstRequest {
    fn default() -> Self {
        BurstRequest {
            num_messages: 0,
        }
    }
}

impl ros2_client::Message for BurstRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BurstResponse {
    pub actually_burst: u64,
}

impl Default for BurstResponse {
    fn default() -> Self {
        BurstResponse {
            actually_burst: 0,
        }
    }
}

impl ros2_client::Message for BurstResponse {}


pub struct Burst;
impl ros2_client::Service for Burst {
    type Request = BurstRequest;
    type Response = BurstResponse;

    fn request_type_name(&self) -> &str { "BurstRequest" }
    fn response_type_name(&self) -> &str { "BurstResponse" }
}
