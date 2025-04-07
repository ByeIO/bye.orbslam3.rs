use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Object2221 {
    pub id: u16,
    pub age: u16,
    pub prediction_age: u16,
    pub relative_timestamp: u16,
    pub reference_point: crate::ibeo_msgs::msg::Point2Di,
    pub reference_point_sigma: crate::ibeo_msgs::msg::Point2Di,
    pub closest_point: crate::ibeo_msgs::msg::Point2Di,
    pub bounding_box_center: crate::ibeo_msgs::msg::Point2Di,
    pub bounding_box_width: u16,
    pub bounding_box_length: u16,
    pub object_box_center: crate::ibeo_msgs::msg::Point2Di,
    pub object_box_size: crate::ibeo_msgs::msg::Size2D,
    pub object_box_orientation: i16,
    pub absolute_velocity: crate::ibeo_msgs::msg::Point2Di,
    pub absolute_velocity_sigma: crate::ibeo_msgs::msg::Size2D,
    pub relative_velocity: crate::ibeo_msgs::msg::Point2Di,
    pub classification: u8,
    pub classification_age: u16,
    pub classification_certainty: u16,
    pub number_of_contour_points: u16,
    pub contour_point_list: Vec<crate::ibeo_msgs::msg::Point2Di>,
}

impl Object2221 {
    pub const UNCLASSIFIED: u8 = 0;
    pub const UNKNOWN_SMALL: u8 = 1;
    pub const UNKNOWN_BIG: u8 = 2;
    pub const PEDESTRIAN: u8 = 3;
    pub const BIKE: u8 = 4;
    pub const CAR: u8 = 5;
    pub const TRUCK: u8 = 6;
}

impl Default for Object2221 {
    fn default() -> Self {
        Object2221 {
            id: 0,
            age: 0,
            prediction_age: 0,
            relative_timestamp: 0,
            reference_point: crate::ibeo_msgs::msg::Point2Di::default(),
            reference_point_sigma: crate::ibeo_msgs::msg::Point2Di::default(),
            closest_point: crate::ibeo_msgs::msg::Point2Di::default(),
            bounding_box_center: crate::ibeo_msgs::msg::Point2Di::default(),
            bounding_box_width: 0,
            bounding_box_length: 0,
            object_box_center: crate::ibeo_msgs::msg::Point2Di::default(),
            object_box_size: crate::ibeo_msgs::msg::Size2D::default(),
            object_box_orientation: 0,
            absolute_velocity: crate::ibeo_msgs::msg::Point2Di::default(),
            absolute_velocity_sigma: crate::ibeo_msgs::msg::Size2D::default(),
            relative_velocity: crate::ibeo_msgs::msg::Point2Di::default(),
            classification: 0,
            classification_age: 0,
            classification_certainty: 0,
            number_of_contour_points: 0,
            contour_point_list: Vec::new(),
        }
    }
}

impl ros2_client::Message for Object2221 {}
