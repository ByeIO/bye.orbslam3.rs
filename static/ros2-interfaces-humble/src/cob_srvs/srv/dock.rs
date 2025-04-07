use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DockRequest {
    pub frame_id: ::std::string::String,
    pub poses: Vec<crate::geometry_msgs::msg::Pose>,
    pub stop_topic: ::std::string::String,
    pub stop_message_field: ::std::string::String,
    pub stop_compare_value: bool,
    pub dist_threshold: f32,
}

impl Default for DockRequest {
    fn default() -> Self {
        DockRequest {
            frame_id: ::std::string::String::new(),
            poses: Vec::new(),
            stop_topic: ::std::string::String::new(),
            stop_message_field: ::std::string::String::new(),
            stop_compare_value: false,
            dist_threshold: 0.0,
        }
    }
}

impl ros2_client::Message for DockRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DockResponse {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for DockResponse {
    fn default() -> Self {
        DockResponse {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for DockResponse {}


pub struct Dock;
impl ros2_client::Service for Dock {
    type Request = DockRequest;
    type Response = DockResponse;

    fn request_type_name(&self) -> &str { "DockRequest" }
    fn response_type_name(&self) -> &str { "DockResponse" }
}
