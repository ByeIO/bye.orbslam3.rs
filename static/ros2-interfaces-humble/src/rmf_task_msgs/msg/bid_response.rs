use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BidResponse {
    pub task_id: ::std::string::String,
    pub has_proposal: bool,
    pub proposal: crate::rmf_task_msgs::msg::BidProposal,
    pub errors: Vec<::std::string::String>,
}

impl Default for BidResponse {
    fn default() -> Self {
        BidResponse {
            task_id: ::std::string::String::new(),
            has_proposal: false,
            proposal: crate::rmf_task_msgs::msg::BidProposal::default(),
            errors: Vec::new(),
        }
    }
}

impl ros2_client::Message for BidResponse {}
