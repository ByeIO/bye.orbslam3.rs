use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListControllersRequest {

}

impl Default for ListControllersRequest {
    fn default() -> Self {
        ListControllersRequest {

        }
    }
}

impl ros2_client::Message for ListControllersRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListControllersResponse {
    pub controller: Vec<crate::controller_manager_msgs::msg::ControllerState>,
}

impl Default for ListControllersResponse {
    fn default() -> Self {
        ListControllersResponse {
            controller: Vec::new(),
        }
    }
}

impl ros2_client::Message for ListControllersResponse {}


pub struct ListControllers;
impl ros2_client::Service for ListControllers {
    type Request = ListControllersRequest;
    type Response = ListControllersResponse;

    fn request_type_name(&self) -> &str { "ListControllersRequest" }
    fn response_type_name(&self) -> &str { "ListControllersResponse" }
}
