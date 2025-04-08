use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeoPoseWithCovarianceStamped {
    pub header: crate::std_msgs::msg::Header,
    pub pose: crate::geographic_msgs::msg::GeoPoseWithCovariance,
}

impl Default for GeoPoseWithCovarianceStamped {
    fn default() -> Self {
        GeoPoseWithCovarianceStamped {
            header: crate::std_msgs::msg::Header::default(),
            pose: crate::geographic_msgs::msg::GeoPoseWithCovariance::default(),
        }
    }
}

impl ros2_client::Message for GeoPoseWithCovarianceStamped {}
