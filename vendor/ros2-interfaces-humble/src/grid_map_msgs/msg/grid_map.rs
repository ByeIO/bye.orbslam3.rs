use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GridMap {
    pub header: crate::std_msgs::msg::Header,
    pub info: crate::grid_map_msgs::msg::GridMapInfo,
    pub layers: Vec<::std::string::String>,
    pub basic_layers: Vec<::std::string::String>,
    pub data: Vec<crate::std_msgs::msg::Float32MultiArray>,
    pub outer_start_index: u16,
    pub inner_start_index: u16,
}

impl Default for GridMap {
    fn default() -> Self {
        GridMap {
            header: crate::std_msgs::msg::Header::default(),
            info: crate::grid_map_msgs::msg::GridMapInfo::default(),
            layers: Vec::new(),
            basic_layers: Vec::new(),
            data: Vec::new(),
            outer_start_index: 0,
            inner_start_index: 0,
        }
    }
}

impl ros2_client::Message for GridMap {}
