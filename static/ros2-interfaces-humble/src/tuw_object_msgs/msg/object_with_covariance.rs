use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObjectWithCovariance {
    pub object: crate::tuw_object_msgs::msg::Object,
    pub covariance_pose: Vec<f64>,
    pub covariance_twist: Vec<f64>,
    pub correlation: Vec<f64>,
}

impl Default for ObjectWithCovariance {
    fn default() -> Self {
        ObjectWithCovariance {
            object: crate::tuw_object_msgs::msg::Object::default(),
            covariance_pose: Vec::new(),
            covariance_twist: Vec::new(),
            correlation: Vec::new(),
        }
    }
}

impl ros2_client::Message for ObjectWithCovariance {}
