use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrackVisual {
    pub header: crate::std_msgs::msg::Header,
    pub name: ::std::string::String,
    pub id: u32,
    pub inherit_orientation: bool,
    pub min_dist: f64,
    pub max_dist: f64,
    pub is_static: bool,
    pub use_model_frame: bool,
    pub xyz: crate::geometry_msgs::msg::Vector3,
    pub inherit_yaw: bool,
}

impl Default for TrackVisual {
    fn default() -> Self {
        TrackVisual {
            header: crate::std_msgs::msg::Header::default(),
            name: ::std::string::String::new(),
            id: 0,
            inherit_orientation: false,
            min_dist: 0.0,
            max_dist: 0.0,
            is_static: false,
            use_model_frame: false,
            xyz: crate::geometry_msgs::msg::Vector3::default(),
            inherit_yaw: false,
        }
    }
}

impl ros2_client::Message for TrackVisual {}
