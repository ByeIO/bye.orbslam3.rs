use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SettingsLoadSaveRequest {
    pub filename: ::std::string::String,
}

impl Default for SettingsLoadSaveRequest {
    fn default() -> Self {
        SettingsLoadSaveRequest {
            filename: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SettingsLoadSaveRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SettingsLoadSaveResponse {
    pub error: crate::vimbax_camera_msgs::msg::Error,
}

impl Default for SettingsLoadSaveResponse {
    fn default() -> Self {
        SettingsLoadSaveResponse {
            error: crate::vimbax_camera_msgs::msg::Error::default(),
        }
    }
}

impl ros2_client::Message for SettingsLoadSaveResponse {}


pub struct SettingsLoadSave;
impl ros2_client::Service for SettingsLoadSave {
    type Request = SettingsLoadSaveRequest;
    type Response = SettingsLoadSaveResponse;

    fn request_type_name(&self) -> &str { "SettingsLoadSaveRequest" }
    fn response_type_name(&self) -> &str { "SettingsLoadSaveResponse" }
}
