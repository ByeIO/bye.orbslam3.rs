use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Object2225 {
    pub id: u16,
    pub age: u32,
    pub timestamp: crate::builtin_interfaces::msg::Time,
    pub hidden_status_age: u16,
    pub classification: u8,
    pub classification_certainty: u8,
    pub classification_age: u32,
    pub bounding_box_center: crate::ibeo_msgs::msg::Point2Df,
    pub bounding_box_size: crate::ibeo_msgs::msg::Point2Df,
    pub object_box_center: crate::ibeo_msgs::msg::Point2Df,
    pub object_box_center_sigma: crate::ibeo_msgs::msg::Point2Df,
    pub object_box_size: crate::ibeo_msgs::msg::Point2Df,
    pub yaw_angle: f32,
    pub relative_velocity: crate::ibeo_msgs::msg::Point2Df,
    pub relative_velocity_sigma: crate::ibeo_msgs::msg::Point2Df,
    pub absolute_velocity: crate::ibeo_msgs::msg::Point2Df,
    pub absolute_velocity_sigma: crate::ibeo_msgs::msg::Point2Df,
    pub number_of_contour_points: u8,
    pub closest_point_index: u8,
    pub contour_point_list: Vec<crate::ibeo_msgs::msg::Point2Df>,
}

impl Object2225 {
    pub const UNCLASSIFIED: u8 = 0;
    pub const UNKNOWN_SMALL: u8 = 1;
    pub const UNKNOWN_BIG: u8 = 2;
    pub const PEDESTRIAN: u8 = 3;
    pub const BIKE: u8 = 4;
    pub const CAR: u8 = 5;
    pub const TRUCK: u8 = 6;
}

impl Default for Object2225 {
    fn default() -> Self {
        Object2225 {
            id: 0,
            age: 0,
            timestamp: crate::builtin_interfaces::msg::Time::default(),
            hidden_status_age: 0,
            classification: 0,
            classification_certainty: 0,
            classification_age: 0,
            bounding_box_center: crate::ibeo_msgs::msg::Point2Df::default(),
            bounding_box_size: crate::ibeo_msgs::msg::Point2Df::default(),
            object_box_center: crate::ibeo_msgs::msg::Point2Df::default(),
            object_box_center_sigma: crate::ibeo_msgs::msg::Point2Df::default(),
            object_box_size: crate::ibeo_msgs::msg::Point2Df::default(),
            yaw_angle: 0.0,
            relative_velocity: crate::ibeo_msgs::msg::Point2Df::default(),
            relative_velocity_sigma: crate::ibeo_msgs::msg::Point2Df::default(),
            absolute_velocity: crate::ibeo_msgs::msg::Point2Df::default(),
            absolute_velocity_sigma: crate::ibeo_msgs::msg::Point2Df::default(),
            number_of_contour_points: 0,
            closest_point_index: 0,
            contour_point_list: Vec::new(),
        }
    }
}

impl ros2_client::Message for Object2225 {}
