use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GridMap {
    pub header: crate::std_msgs::msg::Header,
    pub top_left: crate::geometry_msgs::msg::Point,
    pub top_right: crate::geometry_msgs::msg::Point,
    pub bottom_right: crate::geometry_msgs::msg::Point,
    pub bottom_left: crate::geometry_msgs::msg::Point,
    pub map_names: Vec<::std::string::String>,
    pub map_data: Vec<crate::sensor_msgs::msg::Image>,
}

impl Default for GridMap {
    fn default() -> Self {
        GridMap {
            header: crate::std_msgs::msg::Header::default(),
            top_left: crate::geometry_msgs::msg::Point::default(),
            top_right: crate::geometry_msgs::msg::Point::default(),
            bottom_right: crate::geometry_msgs::msg::Point::default(),
            bottom_left: crate::geometry_msgs::msg::Point::default(),
            map_names: Vec::new(),
            map_data: Vec::new(),
        }
    }
}

impl ros2_client::Message for GridMap {}
