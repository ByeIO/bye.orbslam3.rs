use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StationManagerStationProtocolRequest {
    pub request: ::std::string::String,
    pub station: crate::tuw_multi_robot_msgs::msg::Station,
}

impl StationManagerStationProtocolRequest {
    pub const APPEND: &'static str = "append";
    pub const REMOVE: &'static str = "remove";
}

impl Default for StationManagerStationProtocolRequest {
    fn default() -> Self {
        StationManagerStationProtocolRequest {
            request: ::std::string::String::new(),
            station: crate::tuw_multi_robot_msgs::msg::Station::default(),
        }
    }
}

impl ros2_client::Message for StationManagerStationProtocolRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StationManagerStationProtocolResponse {
    pub response: ::std::string::String,
}

impl StationManagerStationProtocolResponse {
    pub const EXECUTED: &'static str = "executed";
    pub const FAILED: &'static str = "failed";
    pub const ERROR: &'static str = "error";
}

impl Default for StationManagerStationProtocolResponse {
    fn default() -> Self {
        StationManagerStationProtocolResponse {
            response: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for StationManagerStationProtocolResponse {}


pub struct StationManagerStationProtocol;
impl ros2_client::Service for StationManagerStationProtocol {
    type Request = StationManagerStationProtocolRequest;
    type Response = StationManagerStationProtocolResponse;

    fn request_type_name(&self) -> &str { "StationManagerStationProtocolRequest" }
    fn response_type_name(&self) -> &str { "StationManagerStationProtocolResponse" }
}
