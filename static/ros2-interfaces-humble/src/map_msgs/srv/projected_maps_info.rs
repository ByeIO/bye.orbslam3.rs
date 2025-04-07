use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectedMapsInfoRequest {
    pub projected_maps_info: Vec<crate::map_msgs::msg::ProjectedMapInfo>,
}

impl Default for ProjectedMapsInfoRequest {
    fn default() -> Self {
        ProjectedMapsInfoRequest {
            projected_maps_info: Vec::new(),
        }
    }
}

impl ros2_client::Message for ProjectedMapsInfoRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectedMapsInfoResponse {

}

impl Default for ProjectedMapsInfoResponse {
    fn default() -> Self {
        ProjectedMapsInfoResponse {

        }
    }
}

impl ros2_client::Message for ProjectedMapsInfoResponse {}


pub struct ProjectedMapsInfo;
impl ros2_client::Service for ProjectedMapsInfo {
    type Request = ProjectedMapsInfoRequest;
    type Response = ProjectedMapsInfoResponse;

    fn request_type_name(&self) -> &str { "ProjectedMapsInfoRequest" }
    fn response_type_name(&self) -> &str { "ProjectedMapsInfoResponse" }
}
