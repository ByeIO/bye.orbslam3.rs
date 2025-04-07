use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddMapvizDisplayRequest {
    pub name: ::std::string::String,
    #[serde(rename = "type")]    pub type_: ::std::string::String,
    pub draw_order: i32,
    pub visible: bool,
    pub properties: Vec<crate::marti_common_msgs::msg::KeyValue>,
}

impl Default for AddMapvizDisplayRequest {
    fn default() -> Self {
        AddMapvizDisplayRequest {
            name: ::std::string::String::new(),
            type_: ::std::string::String::new(),
            draw_order: 0,
            visible: false,
            properties: Vec::new(),
        }
    }
}

impl ros2_client::Message for AddMapvizDisplayRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddMapvizDisplayResponse {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for AddMapvizDisplayResponse {
    fn default() -> Self {
        AddMapvizDisplayResponse {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for AddMapvizDisplayResponse {}


pub struct AddMapvizDisplay;
impl ros2_client::Service for AddMapvizDisplay {
    type Request = AddMapvizDisplayRequest;
    type Response = AddMapvizDisplayResponse;

    fn request_type_name(&self) -> &str { "AddMapvizDisplayRequest" }
    fn response_type_name(&self) -> &str { "AddMapvizDisplayResponse" }
}
