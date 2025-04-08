use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddStaticTransformRequest {
    pub frame_id: ::std::string::String,
    pub child_frame_id: ::std::string::String,
    pub transform: crate::geometry_msgs::msg::Transform,
}

impl Default for AddStaticTransformRequest {
    fn default() -> Self {
        AddStaticTransformRequest {
            frame_id: ::std::string::String::new(),
            child_frame_id: ::std::string::String::new(),
            transform: crate::geometry_msgs::msg::Transform::default(),
        }
    }
}

impl ros2_client::Message for AddStaticTransformRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddStaticTransformResponse {
    pub success: bool,
}

impl Default for AddStaticTransformResponse {
    fn default() -> Self {
        AddStaticTransformResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for AddStaticTransformResponse {}


pub struct AddStaticTransform;
impl ros2_client::Service for AddStaticTransform {
    type Request = AddStaticTransformRequest;
    type Response = AddStaticTransformResponse;

    fn request_type_name(&self) -> &str { "AddStaticTransformRequest" }
    fn response_type_name(&self) -> &str { "AddStaticTransformResponse" }
}
