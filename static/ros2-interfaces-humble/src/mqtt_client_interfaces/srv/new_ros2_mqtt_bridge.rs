use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NewRos2MqttBridgeRequest {
    pub ros_topic: ::std::string::String,
    pub mqtt_topic: ::std::string::String,
    pub primitive: bool, // default: false
    pub inject_timestamp: bool, // default: false
    pub ros_queue_size: u32, // default: 1
    pub mqtt_qos: u8, // default: 0
    pub mqtt_retained: bool, // default: false
}

impl Default for NewRos2MqttBridgeRequest {
    fn default() -> Self {
        NewRos2MqttBridgeRequest {
            ros_topic: ::std::string::String::new(),
            mqtt_topic: ::std::string::String::new(),
            primitive: false,
            inject_timestamp: false,
            ros_queue_size: 1,
            mqtt_qos: 0,
            mqtt_retained: false,
        }
    }
}

impl ros2_client::Message for NewRos2MqttBridgeRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NewRos2MqttBridgeResponse {
    pub success: bool,
}

impl Default for NewRos2MqttBridgeResponse {
    fn default() -> Self {
        NewRos2MqttBridgeResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for NewRos2MqttBridgeResponse {}


pub struct NewRos2MqttBridge;
impl ros2_client::Service for NewRos2MqttBridge {
    type Request = NewRos2MqttBridgeRequest;
    type Response = NewRos2MqttBridgeResponse;

    fn request_type_name(&self) -> &str { "NewRos2MqttBridgeRequest" }
    fn response_type_name(&self) -> &str { "NewRos2MqttBridgeResponse" }
}
