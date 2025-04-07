use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetPenRequest {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub width: u8,
    pub off: u8,
}

impl Default for SetPenRequest {
    fn default() -> Self {
        SetPenRequest {
            r: 0,
            g: 0,
            b: 0,
            width: 0,
            off: 0,
        }
    }
}

impl ros2_client::Message for SetPenRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetPenResponse {

}

impl Default for SetPenResponse {
    fn default() -> Self {
        SetPenResponse {

        }
    }
}

impl ros2_client::Message for SetPenResponse {}


pub struct SetPen;
impl ros2_client::Service for SetPen {
    type Request = SetPenRequest;
    type Response = SetPenResponse;

    fn request_type_name(&self) -> &str { "SetPenRequest" }
    fn response_type_name(&self) -> &str { "SetPenResponse" }
}
