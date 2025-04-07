use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetPidGainsRequest {
    pub p: f64,
    pub i: f64,
    pub d: f64,
    pub i_clamp: f64,
    pub antiwindup: bool,
}

impl Default for SetPidGainsRequest {
    fn default() -> Self {
        SetPidGainsRequest {
            p: 0.0,
            i: 0.0,
            d: 0.0,
            i_clamp: 0.0,
            antiwindup: false,
        }
    }
}

impl ros2_client::Message for SetPidGainsRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetPidGainsResponse {

}

impl Default for SetPidGainsResponse {
    fn default() -> Self {
        SetPidGainsResponse {

        }
    }
}

impl ros2_client::Message for SetPidGainsResponse {}


pub struct SetPidGains;
impl ros2_client::Service for SetPidGains {
    type Request = SetPidGainsRequest;
    type Response = SetPidGainsResponse;

    fn request_type_name(&self) -> &str { "SetPidGainsRequest" }
    fn response_type_name(&self) -> &str { "SetPidGainsResponse" }
}
