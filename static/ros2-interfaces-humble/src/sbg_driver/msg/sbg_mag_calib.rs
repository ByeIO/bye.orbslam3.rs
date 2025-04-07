use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SbgMagCalib {
    pub header: crate::std_msgs::msg::Header,
}

impl Default for SbgMagCalib {
    fn default() -> Self {
        SbgMagCalib {
            header: crate::std_msgs::msg::Header::default(),
        }
    }
}

impl ros2_client::Message for SbgMagCalib {}
