use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMap2Request {
    pub global_map: bool,
    pub optimized: bool,
    pub with_images: bool,
    pub with_scans: bool,
    pub with_user_data: bool,
    pub with_grids: bool,
    pub with_words: bool,
    pub with_global_descriptors: bool,
}

impl Default for GetMap2Request {
    fn default() -> Self {
        GetMap2Request {
            global_map: false,
            optimized: false,
            with_images: false,
            with_scans: false,
            with_user_data: false,
            with_grids: false,
            with_words: false,
            with_global_descriptors: false,
        }
    }
}

impl ros2_client::Message for GetMap2Request {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMap2Response {
    pub data: crate::rtabmap_msgs::msg::MapData,
}

impl Default for GetMap2Response {
    fn default() -> Self {
        GetMap2Response {
            data: crate::rtabmap_msgs::msg::MapData::default(),
        }
    }
}

impl ros2_client::Message for GetMap2Response {}


pub struct GetMap2;
impl ros2_client::Service for GetMap2 {
    type Request = GetMap2Request;
    type Response = GetMap2Response;

    fn request_type_name(&self) -> &str { "GetMap2Request" }
    fn response_type_name(&self) -> &str { "GetMap2Response" }
}
