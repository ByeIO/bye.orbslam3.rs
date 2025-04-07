use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReconfigureSnapshotStreamRequest {
    pub topic_name: ::std::string::String,
    pub parameters: crate::py_trees_ros_interfaces::msg::SnapshotStreamParameters,
}

impl Default for ReconfigureSnapshotStreamRequest {
    fn default() -> Self {
        ReconfigureSnapshotStreamRequest {
            topic_name: ::std::string::String::new(),
            parameters: crate::py_trees_ros_interfaces::msg::SnapshotStreamParameters::default(),
        }
    }
}

impl ros2_client::Message for ReconfigureSnapshotStreamRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReconfigureSnapshotStreamResponse {
    pub result: bool,
}

impl Default for ReconfigureSnapshotStreamResponse {
    fn default() -> Self {
        ReconfigureSnapshotStreamResponse {
            result: false,
        }
    }
}

impl ros2_client::Message for ReconfigureSnapshotStreamResponse {}


pub struct ReconfigureSnapshotStream;
impl ros2_client::Service for ReconfigureSnapshotStream {
    type Request = ReconfigureSnapshotStreamRequest;
    type Response = ReconfigureSnapshotStreamResponse;

    fn request_type_name(&self) -> &str { "ReconfigureSnapshotStreamRequest" }
    fn response_type_name(&self) -> &str { "ReconfigureSnapshotStreamResponse" }
}
