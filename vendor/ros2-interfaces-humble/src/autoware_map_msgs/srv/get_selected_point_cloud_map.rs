use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSelectedPointCloudMapRequest {
    pub cell_ids: Vec<::std::string::String>,
}

impl Default for GetSelectedPointCloudMapRequest {
    fn default() -> Self {
        GetSelectedPointCloudMapRequest {
            cell_ids: Vec::new(),
        }
    }
}

impl ros2_client::Message for GetSelectedPointCloudMapRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSelectedPointCloudMapResponse {
    pub header: crate::std_msgs::msg::Header,
    pub new_pointcloud_with_ids: Vec<crate::autoware_map_msgs::msg::PointCloudMapCellWithID>,
}

impl Default for GetSelectedPointCloudMapResponse {
    fn default() -> Self {
        GetSelectedPointCloudMapResponse {
            header: crate::std_msgs::msg::Header::default(),
            new_pointcloud_with_ids: Vec::new(),
        }
    }
}

impl ros2_client::Message for GetSelectedPointCloudMapResponse {}


pub struct GetSelectedPointCloudMap;
impl ros2_client::Service for GetSelectedPointCloudMap {
    type Request = GetSelectedPointCloudMapRequest;
    type Response = GetSelectedPointCloudMapResponse;

    fn request_type_name(&self) -> &str { "GetSelectedPointCloudMapRequest" }
    fn response_type_name(&self) -> &str { "GetSelectedPointCloudMapResponse" }
}
