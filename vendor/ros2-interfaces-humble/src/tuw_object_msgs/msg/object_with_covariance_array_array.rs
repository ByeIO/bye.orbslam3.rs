use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObjectWithCovarianceArrayArray {
    pub header: crate::std_msgs::msg::Header,
    pub objects_array: Vec<crate::tuw_object_msgs::msg::ObjectWithCovarianceArray>,
}

impl Default for ObjectWithCovarianceArrayArray {
    fn default() -> Self {
        ObjectWithCovarianceArrayArray {
            header: crate::std_msgs::msg::Header::default(),
            objects_array: Vec::new(),
        }
    }
}

impl ros2_client::Message for ObjectWithCovarianceArrayArray {}
