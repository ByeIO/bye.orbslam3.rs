use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDifferentialPointCloudMapRequest {
    pub area: crate::autoware_map_msgs::msg::AreaInfo,
    pub cached_ids: Vec<::std::string::String>,
}

impl Default for GetDifferentialPointCloudMapRequest {
    fn default() -> Self {
        GetDifferentialPointCloudMapRequest {
            area: crate::autoware_map_msgs::msg::AreaInfo::default(),
            cached_ids: Vec::new(),
        }
    }
}

impl ros2_client::Message for GetDifferentialPointCloudMapRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDifferentialPointCloudMapResponse {
    pub header: crate::std_msgs::msg::Header,
    pub new_pointcloud_with_ids: Vec<crate::autoware_map_msgs::msg::PointCloudMapCellWithID>,
    pub ids_to_remove: Vec<::std::string::String>,
}

impl Default for GetDifferentialPointCloudMapResponse {
    fn default() -> Self {
        GetDifferentialPointCloudMapResponse {
            header: crate::std_msgs::msg::Header::default(),
            new_pointcloud_with_ids: Vec::new(),
            ids_to_remove: Vec::new(),
        }
    }
}

impl ros2_client::Message for GetDifferentialPointCloudMapResponse {}


pub struct GetDifferentialPointCloudMap;
impl ros2_client::Service for GetDifferentialPointCloudMap {
    type Request = GetDifferentialPointCloudMapRequest;
    type Response = GetDifferentialPointCloudMapResponse;

    fn request_type_name(&self) -> &str { "GetDifferentialPointCloudMapRequest" }
    fn response_type_name(&self) -> &str { "GetDifferentialPointCloudMapResponse" }
}
