use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeoreferencingMetadata {
    pub valid: bool,
    pub t_enu_to_map: crate::geometry_msgs::msg::PoseWithCovariance,
    pub t_enu_to_utm: crate::geometry_msgs::msg::Pose,
    pub latitude: f64,
    pub longitude: f64,
    pub height: f64,
    pub utm_zone: i32,
    pub utm_band: ::std::string::String,
}

impl Default for GeoreferencingMetadata {
    fn default() -> Self {
        GeoreferencingMetadata {
            valid: false,
            t_enu_to_map: crate::geometry_msgs::msg::PoseWithCovariance::default(),
            t_enu_to_utm: crate::geometry_msgs::msg::Pose::default(),
            latitude: 0.0,
            longitude: 0.0,
            height: 0.0,
            utm_zone: 0,
            utm_band: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GeoreferencingMetadata {}
