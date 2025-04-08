use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccelStamped {
    pub header: crate::std_msgs::msg::Header,
    pub accel: crate::geometry_msgs::msg::Accel,
}

impl Default for AccelStamped {
    fn default() -> Self {
        AccelStamped {
            header: crate::std_msgs::msg::Header::default(),
            accel: crate::geometry_msgs::msg::Accel::default(),
        }
    }
}

impl ros2_client::Message for AccelStamped {}
