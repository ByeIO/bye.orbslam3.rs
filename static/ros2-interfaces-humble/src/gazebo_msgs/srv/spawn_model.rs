use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpawnModelRequest {
    pub model_name: ::std::string::String,
    pub model_xml: ::std::string::String,
    pub robot_namespace: ::std::string::String,
    pub initial_pose: crate::geometry_msgs::msg::Pose,
    pub reference_frame: ::std::string::String,
}

impl Default for SpawnModelRequest {
    fn default() -> Self {
        SpawnModelRequest {
            model_name: ::std::string::String::new(),
            model_xml: ::std::string::String::new(),
            robot_namespace: ::std::string::String::new(),
            initial_pose: crate::geometry_msgs::msg::Pose::default(),
            reference_frame: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SpawnModelRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpawnModelResponse {
    pub success: bool,
    pub status_message: ::std::string::String,
}

impl Default for SpawnModelResponse {
    fn default() -> Self {
        SpawnModelResponse {
            success: false,
            status_message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SpawnModelResponse {}


pub struct SpawnModel;
impl ros2_client::Service for SpawnModel {
    type Request = SpawnModelRequest;
    type Response = SpawnModelResponse;

    fn request_type_name(&self) -> &str { "SpawnModelRequest" }
    fn response_type_name(&self) -> &str { "SpawnModelResponse" }
}
