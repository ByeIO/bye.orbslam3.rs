use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenSnapshotStreamRequest {
    pub topic_name: ::std::string::String,
    pub parameters: crate::py_trees_ros_interfaces::msg::SnapshotStreamParameters,
}

impl Default for OpenSnapshotStreamRequest {
    fn default() -> Self {
        OpenSnapshotStreamRequest {
            topic_name: ::std::string::String::new(),
            parameters: crate::py_trees_ros_interfaces::msg::SnapshotStreamParameters::default(),
        }
    }
}

impl ros2_client::Message for OpenSnapshotStreamRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenSnapshotStreamResponse {
    pub topic_name: ::std::string::String,
}

impl Default for OpenSnapshotStreamResponse {
    fn default() -> Self {
        OpenSnapshotStreamResponse {
            topic_name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for OpenSnapshotStreamResponse {}


pub struct OpenSnapshotStream;
impl ros2_client::Service for OpenSnapshotStream {
    type Request = OpenSnapshotStreamRequest;
    type Response = OpenSnapshotStreamResponse;

    fn request_type_name(&self) -> &str { "OpenSnapshotStreamRequest" }
    fn response_type_name(&self) -> &str { "OpenSnapshotStreamResponse" }
}
