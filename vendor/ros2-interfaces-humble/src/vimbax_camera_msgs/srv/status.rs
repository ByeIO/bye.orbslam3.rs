use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatusRequest {

}

impl Default for StatusRequest {
    fn default() -> Self {
        StatusRequest {

        }
    }
}

impl ros2_client::Message for StatusRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatusResponse {
    pub error: crate::vimbax_camera_msgs::msg::Error,
    pub display_name: ::std::string::String,
    pub model_name: ::std::string::String,
    pub device_firmware_version: ::std::string::String,
    pub device_id: ::std::string::String,
    pub device_user_id: ::std::string::String,
    pub device_serial_number: ::std::string::String,
    pub interface_id: ::std::string::String,
    pub transport_layer_id: ::std::string::String,
    pub streaming: bool,
    pub width: u32,
    pub height: u32,
    pub frame_rate: f64,
    pub pixel_format: ::std::string::String,
    pub trigger_info: Vec<crate::vimbax_camera_msgs::msg::TriggerInfo>,
    pub ip_address: ::std::string::String,
    pub mac_address: ::std::string::String,
}

impl Default for StatusResponse {
    fn default() -> Self {
        StatusResponse {
            error: crate::vimbax_camera_msgs::msg::Error::default(),
            display_name: ::std::string::String::new(),
            model_name: ::std::string::String::new(),
            device_firmware_version: ::std::string::String::new(),
            device_id: ::std::string::String::new(),
            device_user_id: ::std::string::String::new(),
            device_serial_number: ::std::string::String::new(),
            interface_id: ::std::string::String::new(),
            transport_layer_id: ::std::string::String::new(),
            streaming: false,
            width: 0,
            height: 0,
            frame_rate: 0.0,
            pixel_format: ::std::string::String::new(),
            trigger_info: Vec::new(),
            ip_address: ::std::string::String::new(),
            mac_address: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for StatusResponse {}


pub struct Status;
impl ros2_client::Service for Status {
    type Request = StatusRequest;
    type Response = StatusResponse;

    fn request_type_name(&self) -> &str { "StatusRequest" }
    fn response_type_name(&self) -> &str { "StatusResponse" }
}
