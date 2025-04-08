use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlaneData {
    pub header: crate::std_msgs::msg::Header,
    pub id: i32,
    pub nx: f32,
    pub ny: f32,
    pub nz: f32,
    pub d: f32,
    pub plane_orientation: crate::geometry_msgs::msg::Vector3,
    pub plane_points: Vec<crate::geometry_msgs::msg::Vector3>,
    pub data_source: ::std::string::String,
}

impl Default for PlaneData {
    fn default() -> Self {
        PlaneData {
            header: crate::std_msgs::msg::Header::default(),
            id: 0,
            nx: 0.0,
            ny: 0.0,
            nz: 0.0,
            d: 0.0,
            plane_orientation: crate::geometry_msgs::msg::Vector3::default(),
            plane_points: Vec::new(),
            data_source: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for PlaneData {}
