use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StreamStartStopRequest {

}

impl Default for StreamStartStopRequest {
    fn default() -> Self {
        StreamStartStopRequest {

        }
    }
}

impl ros2_client::Message for StreamStartStopRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StreamStartStopResponse {
    pub error: crate::vimbax_camera_msgs::msg::Error,
}

impl Default for StreamStartStopResponse {
    fn default() -> Self {
        StreamStartStopResponse {
            error: crate::vimbax_camera_msgs::msg::Error::default(),
        }
    }
}

impl ros2_client::Message for StreamStartStopResponse {}


pub struct StreamStartStop;
impl ros2_client::Service for StreamStartStop {
    type Request = StreamStartStopRequest;
    type Response = StreamStartStopResponse;

    fn request_type_name(&self) -> &str { "StreamStartStopRequest" }
    fn response_type_name(&self) -> &str { "StreamStartStopResponse" }
}
