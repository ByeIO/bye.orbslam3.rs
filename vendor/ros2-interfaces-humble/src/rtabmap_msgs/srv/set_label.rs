use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetLabelRequest {
    pub node_id: i32,
    pub node_label: ::std::string::String,
}

impl Default for SetLabelRequest {
    fn default() -> Self {
        SetLabelRequest {
            node_id: 0,
            node_label: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SetLabelRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetLabelResponse {

}

impl Default for SetLabelResponse {
    fn default() -> Self {
        SetLabelResponse {

        }
    }
}

impl ros2_client::Message for SetLabelResponse {}


pub struct SetLabel;
impl ros2_client::Service for SetLabel {
    type Request = SetLabelRequest;
    type Response = SetLabelResponse;

    fn request_type_name(&self) -> &str { "SetLabelRequest" }
    fn response_type_name(&self) -> &str { "SetLabelResponse" }
}
