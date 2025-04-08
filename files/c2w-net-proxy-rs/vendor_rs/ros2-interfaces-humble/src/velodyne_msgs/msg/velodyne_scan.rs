use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VelodyneScan {
    pub header: crate::std_msgs::msg::Header,
    pub packets: Vec<crate::velodyne_msgs::msg::VelodynePacket>,
}

impl Default for VelodyneScan {
    fn default() -> Self {
        VelodyneScan {
            header: crate::std_msgs::msg::Header::default(),
            packets: Vec::new(),
        }
    }
}

impl ros2_client::Message for VelodyneScan {}
