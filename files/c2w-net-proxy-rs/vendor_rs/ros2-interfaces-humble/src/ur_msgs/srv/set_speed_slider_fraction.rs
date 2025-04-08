use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetSpeedSliderFractionRequest {
    pub speed_slider_fraction: f64,
}

impl Default for SetSpeedSliderFractionRequest {
    fn default() -> Self {
        SetSpeedSliderFractionRequest {
            speed_slider_fraction: 0.0,
        }
    }
}

impl ros2_client::Message for SetSpeedSliderFractionRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetSpeedSliderFractionResponse {
    pub success: bool,
}

impl Default for SetSpeedSliderFractionResponse {
    fn default() -> Self {
        SetSpeedSliderFractionResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for SetSpeedSliderFractionResponse {}


pub struct SetSpeedSliderFraction;
impl ros2_client::Service for SetSpeedSliderFraction {
    type Request = SetSpeedSliderFractionRequest;
    type Response = SetSpeedSliderFractionResponse;

    fn request_type_name(&self) -> &str { "SetSpeedSliderFractionRequest" }
    fn response_type_name(&self) -> &str { "SetSpeedSliderFractionResponse" }
}
