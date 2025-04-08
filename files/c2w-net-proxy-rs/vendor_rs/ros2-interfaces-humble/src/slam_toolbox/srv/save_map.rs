use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SaveMapRequest {
    pub name: crate::std_msgs::msg::String,
}

impl Default for SaveMapRequest {
    fn default() -> Self {
        SaveMapRequest {
            name: crate::std_msgs::msg::String::default(),
        }
    }
}

impl ros2_client::Message for SaveMapRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SaveMapResponse {
    pub result: u8,
}

impl SaveMapResponse {
    pub const RESULT_SUCCESS: u8 = 0;
    pub const RESULT_NO_MAP_RECEIEVD: u8 = 1;
    pub const RESULT_UNDEFINED_FAILURE: u8 = 255;
}

impl Default for SaveMapResponse {
    fn default() -> Self {
        SaveMapResponse {
            result: 0,
        }
    }
}

impl ros2_client::Message for SaveMapResponse {}


pub struct SaveMap;
impl ros2_client::Service for SaveMap {
    type Request = SaveMapRequest;
    type Response = SaveMapResponse;

    fn request_type_name(&self) -> &str { "SaveMapRequest" }
    fn response_type_name(&self) -> &str { "SaveMapResponse" }
}
