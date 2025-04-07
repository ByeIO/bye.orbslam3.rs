use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JointRequestRequest {
    pub joint_name: ::std::string::String,
}

impl Default for JointRequestRequest {
    fn default() -> Self {
        JointRequestRequest {
            joint_name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for JointRequestRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JointRequestResponse {

}

impl Default for JointRequestResponse {
    fn default() -> Self {
        JointRequestResponse {

        }
    }
}

impl ros2_client::Message for JointRequestResponse {}


pub struct JointRequest;
impl ros2_client::Service for JointRequest {
    type Request = JointRequestRequest;
    type Response = JointRequestResponse;

    fn request_type_name(&self) -> &str { "JointRequestRequest" }
    fn response_type_name(&self) -> &str { "JointRequestResponse" }
}
