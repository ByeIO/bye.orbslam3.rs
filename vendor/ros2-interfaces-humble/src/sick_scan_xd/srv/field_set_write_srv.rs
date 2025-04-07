use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldSetWriteSrvRequest {
    pub field_set_selection_method_in: i32,
    pub active_field_set_in: i32,
}

impl Default for FieldSetWriteSrvRequest {
    fn default() -> Self {
        FieldSetWriteSrvRequest {
            field_set_selection_method_in: 0,
            active_field_set_in: 0,
        }
    }
}

impl ros2_client::Message for FieldSetWriteSrvRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldSetWriteSrvResponse {
    pub field_set_selection_method: i32,
    pub active_field_set: i32,
    pub success: bool,
}

impl Default for FieldSetWriteSrvResponse {
    fn default() -> Self {
        FieldSetWriteSrvResponse {
            field_set_selection_method: 0,
            active_field_set: 0,
            success: false,
        }
    }
}

impl ros2_client::Message for FieldSetWriteSrvResponse {}


pub struct FieldSetWriteSrv;
impl ros2_client::Service for FieldSetWriteSrv {
    type Request = FieldSetWriteSrvRequest;
    type Response = FieldSetWriteSrvResponse;

    fn request_type_name(&self) -> &str { "FieldSetWriteSrvRequest" }
    fn response_type_name(&self) -> &str { "FieldSetWriteSrvResponse" }
}
