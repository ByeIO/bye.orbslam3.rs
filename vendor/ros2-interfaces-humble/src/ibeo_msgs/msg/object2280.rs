use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Object2280 {
    pub id: u16,
    pub tracking_model: u8,
    pub mobility_of_dyn_object_detected: bool,
    pub motion_model_validated: bool,
    pub object_age: u32,
    pub timestamp: crate::builtin_interfaces::msg::Time,
    pub object_prediction_age: u16,
    pub classification: u8,
    pub classification_certainty: u8,
    pub classification_age: u32,
    pub object_box_center: crate::ibeo_msgs::msg::Point2Df,
    pub object_box_center_sigma: crate::ibeo_msgs::msg::Point2Df,
    pub object_box_size: crate::ibeo_msgs::msg::Point2Df,
    pub object_box_orientation_angle: f32,
    pub object_box_orientation_angle_sigma: f32,
    pub relative_velocity: crate::ibeo_msgs::msg::Point2Df,
    pub relative_velocity_sigma: crate::ibeo_msgs::msg::Point2Df,
    pub absolute_velocity: crate::ibeo_msgs::msg::Point2Df,
    pub absolute_velocity_sigma: crate::ibeo_msgs::msg::Point2Df,
    pub number_of_contour_points: u8,
    pub closest_point_index: u8,
    pub reference_point_location: u16,
    pub reference_point_coordinate: crate::ibeo_msgs::msg::Point2Df,
    pub reference_point_coordinate_sigma: crate::ibeo_msgs::msg::Point2Df,
    pub reference_point_position_correction_coefficient: f32,
    pub object_priority: u16,
    pub object_existence_measurement: f32,
    pub contour_point_list: Vec<crate::ibeo_msgs::msg::Point2Df>,
}

impl Object2280 {
    pub const DYNAMIC_MODEL: u8 = 0;
    pub const STATIC_MODEL: u8 = 1;
    pub const UNCLASSIFIED: u8 = 0;
    pub const UNKNOWN_SMALL: u8 = 1;
    pub const UNKNOWN_BIG: u8 = 2;
    pub const PEDESTRIAN: u8 = 3;
    pub const BIKE: u8 = 4;
    pub const CAR: u8 = 5;
    pub const TRUCK: u8 = 6;
    pub const UNDER_DRIVABLE: u8 = 12;
    pub const OVER_DRIVABLE: u8 = 13;
    pub const CENTER_OF_GRAVITY: u16 = 0;
    pub const FRONT_LEFT: u16 = 1;
    pub const FRONT_RIGHT: u16 = 2;
    pub const REAR_RIGHT: u16 = 3;
    pub const REAR_LEFT: u16 = 4;
    pub const FRONT_CENTER: u16 = 5;
    pub const RIGHT_CENTER: u16 = 6;
    pub const REAR_CENTER: u16 = 7;
    pub const LEFT_CENTER: u16 = 8;
    pub const OBJECT_CENTER: u16 = 9;
    pub const UNKNOWN: u16 = 255;
}

impl Default for Object2280 {
    fn default() -> Self {
        Object2280 {
            id: 0,
            tracking_model: 0,
            mobility_of_dyn_object_detected: false,
            motion_model_validated: false,
            object_age: 0,
            timestamp: crate::builtin_interfaces::msg::Time::default(),
            object_prediction_age: 0,
            classification: 0,
            classification_certainty: 0,
            classification_age: 0,
            object_box_center: crate::ibeo_msgs::msg::Point2Df::default(),
            object_box_center_sigma: crate::ibeo_msgs::msg::Point2Df::default(),
            object_box_size: crate::ibeo_msgs::msg::Point2Df::default(),
            object_box_orientation_angle: 0.0,
            object_box_orientation_angle_sigma: 0.0,
            relative_velocity: crate::ibeo_msgs::msg::Point2Df::default(),
            relative_velocity_sigma: crate::ibeo_msgs::msg::Point2Df::default(),
            absolute_velocity: crate::ibeo_msgs::msg::Point2Df::default(),
            absolute_velocity_sigma: crate::ibeo_msgs::msg::Point2Df::default(),
            number_of_contour_points: 0,
            closest_point_index: 0,
            reference_point_location: 0,
            reference_point_coordinate: crate::ibeo_msgs::msg::Point2Df::default(),
            reference_point_coordinate_sigma: crate::ibeo_msgs::msg::Point2Df::default(),
            reference_point_position_correction_coefficient: 0.0,
            object_priority: 0,
            object_existence_measurement: 0.0,
            contour_point_list: Vec::new(),
        }
    }
}

impl ros2_client::Message for Object2280 {}
