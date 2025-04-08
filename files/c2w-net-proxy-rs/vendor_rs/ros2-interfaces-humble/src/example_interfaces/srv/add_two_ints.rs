use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddTwoIntsRequest {
    pub a: i64,
    pub b: i64,
}

impl Default for AddTwoIntsRequest {
    fn default() -> Self {
        AddTwoIntsRequest {
            a: 0,
            b: 0,
        }
    }
}

impl ros2_client::Message for AddTwoIntsRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddTwoIntsResponse {
    pub sum: i64,
}

impl Default for AddTwoIntsResponse {
    fn default() -> Self {
        AddTwoIntsResponse {
            sum: 0,
        }
    }
}

impl ros2_client::Message for AddTwoIntsResponse {}


pub struct AddTwoInts;
impl ros2_client::Service for AddTwoInts {
    type Request = AddTwoIntsRequest;
    type Response = AddTwoIntsResponse;

    fn request_type_name(&self) -> &str { "AddTwoIntsRequest" }
    fn response_type_name(&self) -> &str { "AddTwoIntsResponse" }
}
