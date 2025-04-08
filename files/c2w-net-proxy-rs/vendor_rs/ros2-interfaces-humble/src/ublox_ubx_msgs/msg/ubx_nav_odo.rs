use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UBXNavOdo {
    pub header: crate::std_msgs::msg::Header,
    pub version: u8,
    pub itow: u32,
    pub distance: u32,
    pub total_distance: u32,
    pub distance_std: u32,
}

impl Default for UBXNavOdo {
    fn default() -> Self {
        UBXNavOdo {
            header: crate::std_msgs::msg::Header::default(),
            version: 0,
            itow: 0,
            distance: 0,
            total_distance: 0,
            distance_std: 0,
        }
    }
}

impl ros2_client::Message for UBXNavOdo {}
