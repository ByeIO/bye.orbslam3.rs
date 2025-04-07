use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegisterQueryRequest {
    pub query: crate::rmf_traffic_msgs::msg::ScheduleQuery,
}

impl Default for RegisterQueryRequest {
    fn default() -> Self {
        RegisterQueryRequest {
            query: crate::rmf_traffic_msgs::msg::ScheduleQuery::default(),
        }
    }
}

impl ros2_client::Message for RegisterQueryRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegisterQueryResponse {
    pub node_id: crate::rmf_traffic_msgs::msg::ScheduleIdentity,
    pub query_id: u64,
    pub error: ::std::string::String,
}

impl Default for RegisterQueryResponse {
    fn default() -> Self {
        RegisterQueryResponse {
            node_id: crate::rmf_traffic_msgs::msg::ScheduleIdentity::default(),
            query_id: 0,
            error: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for RegisterQueryResponse {}


pub struct RegisterQuery;
impl ros2_client::Service for RegisterQuery {
    type Request = RegisterQueryRequest;
    type Response = RegisterQueryResponse;

    fn request_type_name(&self) -> &str { "RegisterQueryRequest" }
    fn response_type_name(&self) -> &str { "RegisterQueryResponse" }
}
