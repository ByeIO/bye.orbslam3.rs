use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandTOLLocalRequest {
    pub min_pitch: f32,
    pub offset: f32,
    pub rate: f32,
    pub yaw: f32,
    pub position: crate::geometry_msgs::msg::Vector3,
}

impl Default for CommandTOLLocalRequest {
    fn default() -> Self {
        CommandTOLLocalRequest {
            min_pitch: 0.0,
            offset: 0.0,
            rate: 0.0,
            yaw: 0.0,
            position: crate::geometry_msgs::msg::Vector3::default(),
        }
    }
}

impl ros2_client::Message for CommandTOLLocalRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandTOLLocalResponse {
    pub success: bool,
    pub result: u8,
}

impl Default for CommandTOLLocalResponse {
    fn default() -> Self {
        CommandTOLLocalResponse {
            success: false,
            result: 0,
        }
    }
}

impl ros2_client::Message for CommandTOLLocalResponse {}


pub struct CommandTOLLocal;
impl ros2_client::Service for CommandTOLLocal {
    type Request = CommandTOLLocalRequest;
    type Response = CommandTOLLocalResponse;

    fn request_type_name(&self) -> &str { "CommandTOLLocalRequest" }
    fn response_type_name(&self) -> &str { "CommandTOLLocalResponse" }
}
