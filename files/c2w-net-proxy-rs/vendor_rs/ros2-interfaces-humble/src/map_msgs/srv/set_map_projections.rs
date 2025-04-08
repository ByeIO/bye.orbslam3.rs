use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetMapProjectionsRequest {

}

impl Default for SetMapProjectionsRequest {
    fn default() -> Self {
        SetMapProjectionsRequest {

        }
    }
}

impl ros2_client::Message for SetMapProjectionsRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetMapProjectionsResponse {
    pub projected_maps_info: Vec<crate::map_msgs::msg::ProjectedMapInfo>,
}

impl Default for SetMapProjectionsResponse {
    fn default() -> Self {
        SetMapProjectionsResponse {
            projected_maps_info: Vec::new(),
        }
    }
}

impl ros2_client::Message for SetMapProjectionsResponse {}


pub struct SetMapProjections;
impl ros2_client::Service for SetMapProjections {
    type Request = SetMapProjectionsRequest;
    type Response = SetMapProjectionsResponse;

    fn request_type_name(&self) -> &str { "SetMapProjectionsRequest" }
    fn response_type_name(&self) -> &str { "SetMapProjectionsResponse" }
}
