use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetLoadCarriersRequest {
    pub load_carrier_ids: Vec<::std::string::String>,
}

impl Default for GetLoadCarriersRequest {
    fn default() -> Self {
        GetLoadCarriersRequest {
            load_carrier_ids: Vec::new(),
        }
    }
}

impl ros2_client::Message for GetLoadCarriersRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetLoadCarriersResponse {
    pub load_carriers: Vec<crate::rc_reason_msgs::msg::LoadCarrierModel>,
    pub return_code: crate::rc_common_msgs::msg::ReturnCode,
}

impl Default for GetLoadCarriersResponse {
    fn default() -> Self {
        GetLoadCarriersResponse {
            load_carriers: Vec::new(),
            return_code: crate::rc_common_msgs::msg::ReturnCode::default(),
        }
    }
}

impl ros2_client::Message for GetLoadCarriersResponse {}


pub struct GetLoadCarriers;
impl ros2_client::Service for GetLoadCarriers {
    type Request = GetLoadCarriersRequest;
    type Response = GetLoadCarriersResponse;

    fn request_type_name(&self) -> &str { "GetLoadCarriersRequest" }
    fn response_type_name(&self) -> &str { "GetLoadCarriersResponse" }
}
