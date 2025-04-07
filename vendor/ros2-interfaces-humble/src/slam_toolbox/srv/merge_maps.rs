use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MergeMapsRequest {

}

impl Default for MergeMapsRequest {
    fn default() -> Self {
        MergeMapsRequest {

        }
    }
}

impl ros2_client::Message for MergeMapsRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MergeMapsResponse {

}

impl Default for MergeMapsResponse {
    fn default() -> Self {
        MergeMapsResponse {

        }
    }
}

impl ros2_client::Message for MergeMapsResponse {}


pub struct MergeMaps;
impl ros2_client::Service for MergeMaps {
    type Request = MergeMapsRequest;
    type Response = MergeMapsResponse;

    fn request_type_name(&self) -> &str { "MergeMapsRequest" }
    fn response_type_name(&self) -> &str { "MergeMapsResponse" }
}
