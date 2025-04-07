use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WallData {
    pub header: crate::std_msgs::msg::Header,
    pub id: i32,
    pub wall_center: crate::geometry_msgs::msg::Pose,
    pub wall_point: crate::geometry_msgs::msg::Point,
    pub x_planes: Vec<crate::situational_graphs_msgs::msg::PlaneData>,
    pub y_planes: Vec<crate::situational_graphs_msgs::msg::PlaneData>,
}

impl Default for WallData {
    fn default() -> Self {
        WallData {
            header: crate::std_msgs::msg::Header::default(),
            id: 0,
            wall_center: crate::geometry_msgs::msg::Pose::default(),
            wall_point: crate::geometry_msgs::msg::Point::default(),
            x_planes: Vec::new(),
            y_planes: Vec::new(),
        }
    }
}

impl ros2_client::Message for WallData {}
