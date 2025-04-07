use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarkerDetection {
    pub header: crate::std_msgs::msg::Header,
    pub distance_min: f32,
    pub distance_max: f32,
    pub distance_max_id: f32,
    pub view_direction: crate::geometry_msgs::msg::Quaternion,
    pub fov_horizontal: f32,
    pub fov_vertical: f32,
    #[serde(rename = "type")]    pub type_: ::std::string::String,
    pub markers: Vec<crate::marker_msgs::msg::Marker>,
}

impl Default for MarkerDetection {
    fn default() -> Self {
        MarkerDetection {
            header: crate::std_msgs::msg::Header::default(),
            distance_min: 0.0,
            distance_max: 0.0,
            distance_max_id: 0.0,
            view_direction: crate::geometry_msgs::msg::Quaternion::default(),
            fov_horizontal: 0.0,
            fov_vertical: 0.0,
            type_: ::std::string::String::new(),
            markers: Vec::new(),
        }
    }
}

impl ros2_client::Message for MarkerDetection {}
