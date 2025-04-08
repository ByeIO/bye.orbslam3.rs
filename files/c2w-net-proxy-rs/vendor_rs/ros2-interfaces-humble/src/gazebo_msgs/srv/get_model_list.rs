use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetModelListRequest {

}

impl Default for GetModelListRequest {
    fn default() -> Self {
        GetModelListRequest {

        }
    }
}

impl ros2_client::Message for GetModelListRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetModelListResponse {
    pub header: crate::std_msgs::msg::Header,
    pub model_names: Vec<::std::string::String>,
    pub success: bool,
}

impl Default for GetModelListResponse {
    fn default() -> Self {
        GetModelListResponse {
            header: crate::std_msgs::msg::Header::default(),
            model_names: Vec::new(),
            success: false,
        }
    }
}

impl ros2_client::Message for GetModelListResponse {}


pub struct GetModelList;
impl ros2_client::Service for GetModelList {
    type Request = GetModelListRequest;
    type Response = GetModelListResponse;

    fn request_type_name(&self) -> &str { "GetModelListRequest" }
    fn response_type_name(&self) -> &str { "GetModelListResponse" }
}
