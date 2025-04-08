use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldDataRequest {

}

impl Default for FieldDataRequest {
    fn default() -> Self {
        FieldDataRequest {

        }
    }
}

impl ros2_client::Message for FieldDataRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldDataResponse {
    pub fields: Vec<crate::sick_safetyscanners2_interfaces::msg::Field>,
    pub device_name: ::std::string::String,
    pub monitoring_cases: Vec<crate::sick_safetyscanners2_interfaces::msg::MonitoringCase>,
}

impl Default for FieldDataResponse {
    fn default() -> Self {
        FieldDataResponse {
            fields: Vec::new(),
            device_name: ::std::string::String::new(),
            monitoring_cases: Vec::new(),
        }
    }
}

impl ros2_client::Message for FieldDataResponse {}


pub struct FieldData;
impl ros2_client::Service for FieldData {
    type Request = FieldDataRequest;
    type Response = FieldDataResponse;

    fn request_type_name(&self) -> &str { "FieldDataRequest" }
    fn response_type_name(&self) -> &str { "FieldDataResponse" }
}
