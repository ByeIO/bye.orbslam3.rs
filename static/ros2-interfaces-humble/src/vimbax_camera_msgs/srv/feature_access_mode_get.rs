use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeatureAccessModeGetRequest {
    pub feature_name: ::std::string::String,
    pub feature_module: crate::vimbax_camera_msgs::msg::FeatureModule,
}

impl Default for FeatureAccessModeGetRequest {
    fn default() -> Self {
        FeatureAccessModeGetRequest {
            feature_name: ::std::string::String::new(),
            feature_module: crate::vimbax_camera_msgs::msg::FeatureModule::default(),
        }
    }
}

impl ros2_client::Message for FeatureAccessModeGetRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeatureAccessModeGetResponse {
    pub is_readable: bool,
    pub is_writeable: bool,
    pub error: crate::vimbax_camera_msgs::msg::Error,
}

impl Default for FeatureAccessModeGetResponse {
    fn default() -> Self {
        FeatureAccessModeGetResponse {
            is_readable: false,
            is_writeable: false,
            error: crate::vimbax_camera_msgs::msg::Error::default(),
        }
    }
}

impl ros2_client::Message for FeatureAccessModeGetResponse {}


pub struct FeatureAccessModeGet;
impl ros2_client::Service for FeatureAccessModeGet {
    type Request = FeatureAccessModeGetRequest;
    type Response = FeatureAccessModeGetResponse;

    fn request_type_name(&self) -> &str { "FeatureAccessModeGetRequest" }
    fn response_type_name(&self) -> &str { "FeatureAccessModeGetResponse" }
}
