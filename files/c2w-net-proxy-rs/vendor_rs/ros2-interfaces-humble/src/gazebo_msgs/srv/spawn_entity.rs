use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpawnEntityRequest {
    pub name: ::std::string::String,
    pub xml: ::std::string::String,
    pub robot_namespace: ::std::string::String,
    pub initial_pose: crate::geometry_msgs::msg::Pose,
    pub reference_frame: ::std::string::String,
}

impl Default for SpawnEntityRequest {
    fn default() -> Self {
        SpawnEntityRequest {
            name: ::std::string::String::new(),
            xml: ::std::string::String::new(),
            robot_namespace: ::std::string::String::new(),
            initial_pose: crate::geometry_msgs::msg::Pose::default(),
            reference_frame: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SpawnEntityRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpawnEntityResponse {
    pub success: bool,
    pub status_message: ::std::string::String,
}

impl Default for SpawnEntityResponse {
    fn default() -> Self {
        SpawnEntityResponse {
            success: false,
            status_message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SpawnEntityResponse {}


pub struct SpawnEntity;
impl ros2_client::Service for SpawnEntity {
    type Request = SpawnEntityRequest;
    type Response = SpawnEntityResponse;

    fn request_type_name(&self) -> &str { "SpawnEntityRequest" }
    fn response_type_name(&self) -> &str { "SpawnEntityResponse" }
}
