use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CalculateWhiteBalanceRequest {

}

impl Default for CalculateWhiteBalanceRequest {
    fn default() -> Self {
        CalculateWhiteBalanceRequest {

        }
    }
}

impl ros2_client::Message for CalculateWhiteBalanceRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CalculateWhiteBalanceResponse {
    pub is_successful: bool,
    pub balance_ratios: Vec<crate::diagnostic_msgs::msg::KeyValue>,
}

impl Default for CalculateWhiteBalanceResponse {
    fn default() -> Self {
        CalculateWhiteBalanceResponse {
            is_successful: false,
            balance_ratios: Vec::new(),
        }
    }
}

impl ros2_client::Message for CalculateWhiteBalanceResponse {}


pub struct CalculateWhiteBalance;
impl ros2_client::Service for CalculateWhiteBalance {
    type Request = CalculateWhiteBalanceRequest;
    type Response = CalculateWhiteBalanceResponse;

    fn request_type_name(&self) -> &str { "CalculateWhiteBalanceRequest" }
    fn response_type_name(&self) -> &str { "CalculateWhiteBalanceResponse" }
}
