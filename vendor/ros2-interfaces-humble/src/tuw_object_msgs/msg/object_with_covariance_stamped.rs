use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObjectWithCovarianceStamped {
    pub header: crate::std_msgs::msg::Header,
    pub object: crate::tuw_object_msgs::msg::ObjectWithCovariance,
}

impl Default for ObjectWithCovarianceStamped {
    fn default() -> Self {
        ObjectWithCovarianceStamped {
            header: crate::std_msgs::msg::Header::default(),
            object: crate::tuw_object_msgs::msg::ObjectWithCovariance::default(),
        }
    }
}

impl ros2_client::Message for ObjectWithCovarianceStamped {}
