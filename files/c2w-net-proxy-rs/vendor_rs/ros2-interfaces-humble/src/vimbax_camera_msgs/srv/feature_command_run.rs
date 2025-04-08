use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeatureCommandRunRequest {
    pub feature_name: ::std::string::String,
    pub feature_module: crate::vimbax_camera_msgs::msg::FeatureModule,
}

impl Default for FeatureCommandRunRequest {
    fn default() -> Self {
        FeatureCommandRunRequest {
            feature_name: ::std::string::String::new(),
            feature_module: crate::vimbax_camera_msgs::msg::FeatureModule::default(),
        }
    }
}

impl ros2_client::Message for FeatureCommandRunRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeatureCommandRunResponse {
    pub error: crate::vimbax_camera_msgs::msg::Error,
}

impl Default for FeatureCommandRunResponse {
    fn default() -> Self {
        FeatureCommandRunResponse {
            error: crate::vimbax_camera_msgs::msg::Error::default(),
        }
    }
}

impl ros2_client::Message for FeatureCommandRunResponse {}


pub struct FeatureCommandRun;
impl ros2_client::Service for FeatureCommandRun {
    type Request = FeatureCommandRunRequest;
    type Response = FeatureCommandRunResponse;

    fn request_type_name(&self) -> &str { "FeatureCommandRunRequest" }
    fn response_type_name(&self) -> &str { "FeatureCommandRunResponse" }
}
