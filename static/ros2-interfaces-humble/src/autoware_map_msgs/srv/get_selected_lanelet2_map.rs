use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSelectedLanelet2MapRequest {
    pub cell_ids: Vec<::std::string::String>,
}

impl Default for GetSelectedLanelet2MapRequest {
    fn default() -> Self {
        GetSelectedLanelet2MapRequest {
            cell_ids: Vec::new(),
        }
    }
}

impl ros2_client::Message for GetSelectedLanelet2MapRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSelectedLanelet2MapResponse {
    pub header: crate::std_msgs::msg::Header,
    pub lanelet2_cells: crate::autoware_map_msgs::msg::LaneletMapBin,
}

impl Default for GetSelectedLanelet2MapResponse {
    fn default() -> Self {
        GetSelectedLanelet2MapResponse {
            header: crate::std_msgs::msg::Header::default(),
            lanelet2_cells: crate::autoware_map_msgs::msg::LaneletMapBin::default(),
        }
    }
}

impl ros2_client::Message for GetSelectedLanelet2MapResponse {}


pub struct GetSelectedLanelet2Map;
impl ros2_client::Service for GetSelectedLanelet2Map {
    type Request = GetSelectedLanelet2MapRequest;
    type Response = GetSelectedLanelet2MapResponse;

    fn request_type_name(&self) -> &str { "GetSelectedLanelet2MapRequest" }
    fn response_type_name(&self) -> &str { "GetSelectedLanelet2MapResponse" }
}
