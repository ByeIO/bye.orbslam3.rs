use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TmcGgpAllRequest {

}

impl Default for TmcGgpAllRequest {
    fn default() -> Self {
        TmcGgpAllRequest {

        }
    }
}

impl ros2_client::Message for TmcGgpAllRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TmcGgpAllResponse {
    pub success: bool,
    pub param: Vec<crate::adi_tmcl::msg::TmcParam>,
}

impl Default for TmcGgpAllResponse {
    fn default() -> Self {
        TmcGgpAllResponse {
            success: false,
            param: Vec::new(),
        }
    }
}

impl ros2_client::Message for TmcGgpAllResponse {}


pub struct TmcGgpAll;
impl ros2_client::Service for TmcGgpAll {
    type Request = TmcGgpAllRequest;
    type Response = TmcGgpAllResponse;

    fn request_type_name(&self) -> &str { "TmcGgpAllRequest" }
    fn response_type_name(&self) -> &str { "TmcGgpAllResponse" }
}
