use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NewMqtt2RosBridgeRequest {
    pub ros_topic: ::std::string::String,
    pub mqtt_topic: ::std::string::String,
    pub primitive: bool, // default: false
    pub mqtt_qos: u8, // default: 0
    pub ros_queue_size: u32, // default: 1
    pub ros_latched: bool, // default: false
}

impl Default for NewMqtt2RosBridgeRequest {
    fn default() -> Self {
        NewMqtt2RosBridgeRequest {
            ros_topic: ::std::string::String::new(),
            mqtt_topic: ::std::string::String::new(),
            primitive: false,
            mqtt_qos: 0,
            ros_queue_size: 1,
            ros_latched: false,
        }
    }
}

impl ros2_client::Message for NewMqtt2RosBridgeRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NewMqtt2RosBridgeResponse {
    pub success: bool,
}

impl Default for NewMqtt2RosBridgeResponse {
    fn default() -> Self {
        NewMqtt2RosBridgeResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for NewMqtt2RosBridgeResponse {}


pub struct NewMqtt2RosBridge;
impl ros2_client::Service for NewMqtt2RosBridge {
    type Request = NewMqtt2RosBridgeRequest;
    type Response = NewMqtt2RosBridgeResponse;

    fn request_type_name(&self) -> &str { "NewMqtt2RosBridgeRequest" }
    fn response_type_name(&self) -> &str { "NewMqtt2RosBridgeResponse" }
}
