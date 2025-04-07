use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListControllerTypesRequest {

}

impl Default for ListControllerTypesRequest {
    fn default() -> Self {
        ListControllerTypesRequest {

        }
    }
}

impl ros2_client::Message for ListControllerTypesRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListControllerTypesResponse {
    pub types: Vec<::std::string::String>,
    pub base_classes: Vec<::std::string::String>,
}

impl Default for ListControllerTypesResponse {
    fn default() -> Self {
        ListControllerTypesResponse {
            types: Vec::new(),
            base_classes: Vec::new(),
        }
    }
}

impl ros2_client::Message for ListControllerTypesResponse {}


pub struct ListControllerTypes;
impl ros2_client::Service for ListControllerTypes {
    type Request = ListControllerTypesRequest;
    type Response = ListControllerTypesResponse;

    fn request_type_name(&self) -> &str { "ListControllerTypesRequest" }
    fn response_type_name(&self) -> &str { "ListControllerTypesResponse" }
}
