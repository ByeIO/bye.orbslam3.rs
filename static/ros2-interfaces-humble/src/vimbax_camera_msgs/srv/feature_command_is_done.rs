use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeatureCommandIsDoneRequest {
    pub feature_name: ::std::string::String,
    pub feature_module: crate::vimbax_camera_msgs::msg::FeatureModule,
}

impl Default for FeatureCommandIsDoneRequest {
    fn default() -> Self {
        FeatureCommandIsDoneRequest {
            feature_name: ::std::string::String::new(),
            feature_module: crate::vimbax_camera_msgs::msg::FeatureModule::default(),
        }
    }
}

impl ros2_client::Message for FeatureCommandIsDoneRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeatureCommandIsDoneResponse {
    pub is_done: bool,
    pub error: crate::vimbax_camera_msgs::msg::Error,
}

impl Default for FeatureCommandIsDoneResponse {
    fn default() -> Self {
        FeatureCommandIsDoneResponse {
            is_done: false,
            error: crate::vimbax_camera_msgs::msg::Error::default(),
        }
    }
}

impl ros2_client::Message for FeatureCommandIsDoneResponse {}


pub struct FeatureCommandIsDone;
impl ros2_client::Service for FeatureCommandIsDone {
    type Request = FeatureCommandIsDoneRequest;
    type Response = FeatureCommandIsDoneResponse;

    fn request_type_name(&self) -> &str { "FeatureCommandIsDoneRequest" }
    fn response_type_name(&self) -> &str { "FeatureCommandIsDoneResponse" }
}
