use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StartGmcmRecordingRequest {
    pub cameras: crate::gazebo_video_monitor_interfaces::msg::Strings,
}

impl Default for StartGmcmRecordingRequest {
    fn default() -> Self {
        StartGmcmRecordingRequest {
            cameras: crate::gazebo_video_monitor_interfaces::msg::Strings::default(),
        }
    }
}

impl ros2_client::Message for StartGmcmRecordingRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StartGmcmRecordingResponse {

}

impl Default for StartGmcmRecordingResponse {
    fn default() -> Self {
        StartGmcmRecordingResponse {

        }
    }
}

impl ros2_client::Message for StartGmcmRecordingResponse {}


pub struct StartGmcmRecording;
impl ros2_client::Service for StartGmcmRecording {
    type Request = StartGmcmRecordingRequest;
    type Response = StartGmcmRecordingResponse;

    fn request_type_name(&self) -> &str { "StartGmcmRecordingRequest" }
    fn response_type_name(&self) -> &str { "StartGmcmRecordingResponse" }
}
