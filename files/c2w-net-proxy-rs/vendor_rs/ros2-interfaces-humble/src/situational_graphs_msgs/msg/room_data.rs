use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RoomData {
    pub header: crate::std_msgs::msg::Header,
    pub id: i32,
    pub neighbour_ids: Vec<i32>,
    pub room_length: crate::geometry_msgs::msg::Point,
    pub room_center: crate::geometry_msgs::msg::Pose,
    pub cluster_center: crate::geometry_msgs::msg::Point,
    pub x_planes: Vec<crate::situational_graphs_msgs::msg::PlaneData>,
    pub y_planes: Vec<crate::situational_graphs_msgs::msg::PlaneData>,
    pub planes: Vec<crate::situational_graphs_msgs::msg::PlaneData>,
    pub cluster_array: crate::visualization_msgs::msg::MarkerArray,
}

impl Default for RoomData {
    fn default() -> Self {
        RoomData {
            header: crate::std_msgs::msg::Header::default(),
            id: 0,
            neighbour_ids: Vec::new(),
            room_length: crate::geometry_msgs::msg::Point::default(),
            room_center: crate::geometry_msgs::msg::Pose::default(),
            cluster_center: crate::geometry_msgs::msg::Point::default(),
            x_planes: Vec::new(),
            y_planes: Vec::new(),
            planes: Vec::new(),
            cluster_array: crate::visualization_msgs::msg::MarkerArray::default(),
        }
    }
}

impl ros2_client::Message for RoomData {}
