use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteLoadCarriersRequest {
    pub load_carrier_ids: Vec<::std::string::String>,
}

impl Default for DeleteLoadCarriersRequest {
    fn default() -> Self {
        DeleteLoadCarriersRequest {
            load_carrier_ids: Vec::new(),
        }
    }
}

impl ros2_client::Message for DeleteLoadCarriersRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteLoadCarriersResponse {
    pub return_code: crate::rc_common_msgs::msg::ReturnCode,
}

impl Default for DeleteLoadCarriersResponse {
    fn default() -> Self {
        DeleteLoadCarriersResponse {
            return_code: crate::rc_common_msgs::msg::ReturnCode::default(),
        }
    }
}

impl ros2_client::Message for DeleteLoadCarriersResponse {}


pub struct DeleteLoadCarriers;
impl ros2_client::Service for DeleteLoadCarriers {
    type Request = DeleteLoadCarriersRequest;
    type Response = DeleteLoadCarriersResponse;

    fn request_type_name(&self) -> &str { "DeleteLoadCarriersRequest" }
    fn response_type_name(&self) -> &str { "DeleteLoadCarriersResponse" }
}
