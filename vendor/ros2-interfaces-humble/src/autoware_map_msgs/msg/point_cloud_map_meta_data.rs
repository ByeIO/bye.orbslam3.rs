use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PointCloudMapMetaData {
    pub header: crate::std_msgs::msg::Header,
    pub metadata_list: Vec<crate::autoware_map_msgs::msg::PointCloudMapCellMetaDataWithID>,
}

impl Default for PointCloudMapMetaData {
    fn default() -> Self {
        PointCloudMapMetaData {
            header: crate::std_msgs::msg::Header::default(),
            metadata_list: Vec::new(),
        }
    }
}

impl ros2_client::Message for PointCloudMapMetaData {}
