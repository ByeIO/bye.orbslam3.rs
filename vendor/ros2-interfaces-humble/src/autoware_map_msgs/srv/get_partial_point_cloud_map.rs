use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPartialPointCloudMapRequest {
    pub area: crate::autoware_map_msgs::msg::AreaInfo,
}

impl Default for GetPartialPointCloudMapRequest {
    fn default() -> Self {
        GetPartialPointCloudMapRequest {
            area: crate::autoware_map_msgs::msg::AreaInfo::default(),
        }
    }
}

impl ros2_client::Message for GetPartialPointCloudMapRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPartialPointCloudMapResponse {
    pub header: crate::std_msgs::msg::Header,
    pub new_pointcloud_with_ids: Vec<crate::autoware_map_msgs::msg::PointCloudMapCellWithID>,
}

impl Default for GetPartialPointCloudMapResponse {
    fn default() -> Self {
        GetPartialPointCloudMapResponse {
            header: crate::std_msgs::msg::Header::default(),
            new_pointcloud_with_ids: Vec::new(),
        }
    }
}

impl ros2_client::Message for GetPartialPointCloudMapResponse {}


pub struct GetPartialPointCloudMap;
impl ros2_client::Service for GetPartialPointCloudMap {
    type Request = GetPartialPointCloudMapRequest;
    type Response = GetPartialPointCloudMapResponse;

    fn request_type_name(&self) -> &str { "GetPartialPointCloudMapRequest" }
    fn response_type_name(&self) -> &str { "GetPartialPointCloudMapResponse" }
}
