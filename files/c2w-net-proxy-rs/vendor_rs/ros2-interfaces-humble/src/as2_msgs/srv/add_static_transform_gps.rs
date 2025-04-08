use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddStaticTransformGpsRequest {
    pub frame_id: ::std::string::String,
    pub child_frame_id: ::std::string::String,
    pub gps_position: crate::sensor_msgs::msg::NavSatFix,
    pub azimuth: f32,
    pub elevation: f32,
    pub bank: f32,
}

impl Default for AddStaticTransformGpsRequest {
    fn default() -> Self {
        AddStaticTransformGpsRequest {
            frame_id: ::std::string::String::new(),
            child_frame_id: ::std::string::String::new(),
            gps_position: crate::sensor_msgs::msg::NavSatFix::default(),
            azimuth: 0.0,
            elevation: 0.0,
            bank: 0.0,
        }
    }
}

impl ros2_client::Message for AddStaticTransformGpsRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddStaticTransformGpsResponse {
    pub success: bool,
}

impl Default for AddStaticTransformGpsResponse {
    fn default() -> Self {
        AddStaticTransformGpsResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for AddStaticTransformGpsResponse {}


pub struct AddStaticTransformGps;
impl ros2_client::Service for AddStaticTransformGps {
    type Request = AddStaticTransformGpsRequest;
    type Response = AddStaticTransformGpsResponse;

    fn request_type_name(&self) -> &str { "AddStaticTransformGpsRequest" }
    fn response_type_name(&self) -> &str { "AddStaticTransformGpsResponse" }
}
