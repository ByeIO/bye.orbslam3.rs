use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetObjectInformationRequest {
    #[serde(rename = "type")]    pub type_: crate::object_recognition_msgs::msg::ObjectType,
}

impl Default for GetObjectInformationRequest {
    fn default() -> Self {
        GetObjectInformationRequest {
            type_: crate::object_recognition_msgs::msg::ObjectType::default(),
        }
    }
}

impl ros2_client::Message for GetObjectInformationRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetObjectInformationResponse {
    pub information: crate::object_recognition_msgs::msg::ObjectInformation,
}

impl Default for GetObjectInformationResponse {
    fn default() -> Self {
        GetObjectInformationResponse {
            information: crate::object_recognition_msgs::msg::ObjectInformation::default(),
        }
    }
}

impl ros2_client::Message for GetObjectInformationResponse {}


pub struct GetObjectInformation;
impl ros2_client::Service for GetObjectInformation {
    type Request = GetObjectInformationRequest;
    type Response = GetObjectInformationResponse;

    fn request_type_name(&self) -> &str { "GetObjectInformationRequest" }
    fn response_type_name(&self) -> &str { "GetObjectInformationResponse" }
}
