use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeviceInfoRequest {

}

impl Default for DeviceInfoRequest {
    fn default() -> Self {
        DeviceInfoRequest {

        }
    }
}

impl ros2_client::Message for DeviceInfoRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeviceInfoResponse {
    pub device_name: ::std::string::String,
    pub serial_number: ::std::string::String,
    pub firmware_version: ::std::string::String,
    pub usb_type_descriptor: ::std::string::String,
    pub firmware_update_id: ::std::string::String,
    pub sensors: ::std::string::String,
    pub physical_port: ::std::string::String,
}

impl Default for DeviceInfoResponse {
    fn default() -> Self {
        DeviceInfoResponse {
            device_name: ::std::string::String::new(),
            serial_number: ::std::string::String::new(),
            firmware_version: ::std::string::String::new(),
            usb_type_descriptor: ::std::string::String::new(),
            firmware_update_id: ::std::string::String::new(),
            sensors: ::std::string::String::new(),
            physical_port: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for DeviceInfoResponse {}


pub struct DeviceInfo;
impl ros2_client::Service for DeviceInfo {
    type Request = DeviceInfoRequest;
    type Response = DeviceInfoResponse;

    fn request_type_name(&self) -> &str { "DeviceInfoRequest" }
    fn response_type_name(&self) -> &str { "DeviceInfoResponse" }
}
