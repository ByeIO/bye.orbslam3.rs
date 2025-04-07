use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GnssInsOrientationStamped {
    pub header: crate::std_msgs::msg::Header,
    pub orientation: crate::autoware_sensing_msgs::msg::GnssInsOrientation,
}

impl Default for GnssInsOrientationStamped {
    fn default() -> Self {
        GnssInsOrientationStamped {
            header: crate::std_msgs::msg::Header::default(),
            orientation: crate::autoware_sensing_msgs::msg::GnssInsOrientation::default(),
        }
    }
}

impl ros2_client::Message for GnssInsOrientationStamped {}
