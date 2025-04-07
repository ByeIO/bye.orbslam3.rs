use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Acro {
    pub header: crate::std_msgs::msg::Header,
    pub angular_rates: crate::geometry_msgs::msg::Vector3,
    pub thrust: crate::geometry_msgs::msg::Vector3,
}

impl Default for Acro {
    fn default() -> Self {
        Acro {
            header: crate::std_msgs::msg::Header::default(),
            angular_rates: crate::geometry_msgs::msg::Vector3::default(),
            thrust: crate::geometry_msgs::msg::Vector3::default(),
        }
    }
}

impl ros2_client::Message for Acro {}
