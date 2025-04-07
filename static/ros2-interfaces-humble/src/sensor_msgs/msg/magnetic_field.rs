use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MagneticField {
    pub header: crate::std_msgs::msg::Header,
    pub magnetic_field: crate::geometry_msgs::msg::Vector3,
    pub magnetic_field_covariance: [f64; 9],
}

impl Default for MagneticField {
    fn default() -> Self {
        MagneticField {
            header: crate::std_msgs::msg::Header::default(),
            magnetic_field: crate::geometry_msgs::msg::Vector3::default(),
            magnetic_field_covariance: [0.0; 9],
        }
    }
}

impl ros2_client::Message for MagneticField {}
