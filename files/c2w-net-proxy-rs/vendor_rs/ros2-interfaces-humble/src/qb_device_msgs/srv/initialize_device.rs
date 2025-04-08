use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InitializeDeviceRequest {
    pub id: i32,
    pub activate: bool,
    pub rescan: bool,
    pub max_repeats: i32,
}

impl Default for InitializeDeviceRequest {
    fn default() -> Self {
        InitializeDeviceRequest {
            id: 0,
            activate: false,
            rescan: false,
            max_repeats: 0,
        }
    }
}

impl ros2_client::Message for InitializeDeviceRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InitializeDeviceResponse {
    pub success: bool,
    pub failures: i32,
    pub message: ::std::string::String,
    pub info: crate::qb_device_msgs::msg::Info,
}

impl Default for InitializeDeviceResponse {
    fn default() -> Self {
        InitializeDeviceResponse {
            success: false,
            failures: 0,
            message: ::std::string::String::new(),
            info: crate::qb_device_msgs::msg::Info::default(),
        }
    }
}

impl ros2_client::Message for InitializeDeviceResponse {}


pub struct InitializeDevice;
impl ros2_client::Service for InitializeDevice {
    type Request = InitializeDeviceRequest;
    type Response = InitializeDeviceResponse;

    fn request_type_name(&self) -> &str { "InitializeDeviceRequest" }
    fn response_type_name(&self) -> &str { "InitializeDeviceResponse" }
}
