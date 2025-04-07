use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldSetReadSrvRequest {

}

impl Default for FieldSetReadSrvRequest {
    fn default() -> Self {
        FieldSetReadSrvRequest {

        }
    }
}

impl ros2_client::Message for FieldSetReadSrvRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldSetReadSrvResponse {
    pub field_set_selection_method: i32,
    pub active_field_set: i32,
    pub success: bool,
}

impl Default for FieldSetReadSrvResponse {
    fn default() -> Self {
        FieldSetReadSrvResponse {
            field_set_selection_method: 0,
            active_field_set: 0,
            success: false,
        }
    }
}

impl ros2_client::Message for FieldSetReadSrvResponse {}


pub struct FieldSetReadSrv;
impl ros2_client::Service for FieldSetReadSrv {
    type Request = FieldSetReadSrvRequest;
    type Response = FieldSetReadSrvResponse;

    fn request_type_name(&self) -> &str { "FieldSetReadSrvRequest" }
    fn response_type_name(&self) -> &str { "FieldSetReadSrvResponse" }
}
