use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RelocalizeFromGNSSRequest {

}

impl Default for RelocalizeFromGNSSRequest {
    fn default() -> Self {
        RelocalizeFromGNSSRequest {

        }
    }
}

impl ros2_client::Message for RelocalizeFromGNSSRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RelocalizeFromGNSSResponse {
    pub accepted: bool,
}

impl Default for RelocalizeFromGNSSResponse {
    fn default() -> Self {
        RelocalizeFromGNSSResponse {
            accepted: false,
        }
    }
}

impl ros2_client::Message for RelocalizeFromGNSSResponse {}


pub struct RelocalizeFromGNSS;
impl ros2_client::Service for RelocalizeFromGNSS {
    type Request = RelocalizeFromGNSSRequest;
    type Response = RelocalizeFromGNSSResponse;

    fn request_type_name(&self) -> &str { "RelocalizeFromGNSSRequest" }
    fn response_type_name(&self) -> &str { "RelocalizeFromGNSSResponse" }
}
