use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Object2270 {
    pub id: u16,
    pub age: u16,
    pub prediction_age: u16,
    pub relative_moment_of_measurement: u16,
    pub reference_point_location: u8,
    pub reference_point_position_x: i16,
    pub reference_point_position_y: i16,
    pub reference_point_position_sigma_x: u16,
    pub reference_point_position_sigma_y: u16,
    pub contour_points_cog_x: i16,
    pub contour_points_cog_y: i16,
    pub object_box_length: u16,
    pub object_box_width: u16,
    pub object_box_orientation_angle: i16,
    pub object_box_orientation_angle_sigma: i16,
    pub absolute_velocity_x: i16,
    pub absolute_velocity_y: i16,
    pub absolute_velocity_sigma_x: u16,
    pub absolute_velocity_sigma_y: u16,
    pub relative_velocity_x: i16,
    pub relative_velocity_y: i16,
    pub relative_velocity_sigma_x: u16,
    pub relative_velocity_sigma_y: u16,
    pub classification: u8,
    pub tracking_model: u8,
    pub mobile_detected: bool,
    pub track_valid: bool,
    pub classification_age: u16,
    pub classification_confidence: u16,
    pub number_of_contour_points: u16,
    pub contour_point_list: Vec<crate::ibeo_msgs::msg::Point2Di>,
}

impl Object2270 {
    pub const CENTER_OF_GRAVITY: u8 = 0;
    pub const TOP_FRONT_LEFT_CORNER: u8 = 1;
    pub const TOP_FRONT_RIGHT_CORNER: u8 = 2;
    pub const BOTTOM_REAR_RIGHT_CORNER: u8 = 3;
    pub const BOTTOM_REAR_LEFT_CORNER: u8 = 4;
    pub const CENTER_OF_TOP_FRONT_EDGE: u8 = 5;
    pub const CENTER_OF_RIGHT_EDGE: u8 = 6;
    pub const CENTER_OF_BOTTOM_REAR_EDGE: u8 = 7;
    pub const CENTER_OF_LEFT_EDGE: u8 = 8;
    pub const BOX_CENTER: u8 = 9;
    pub const INVALID: u8 = 255;
    pub const UNCLASSIFIED: u8 = 0;
    pub const UNKNOWN_SMALL: u8 = 1;
    pub const UNKNOWN_BIG: u8 = 2;
    pub const PEDESTRIAN: u8 = 3;
    pub const BIKE: u8 = 4;
    pub const CAR: u8 = 5;
    pub const TRUCK: u8 = 6;
    pub const OVER_DRIVABLE: u8 = 10;
    pub const UNDER_DRIVABLE: u8 = 11;
    pub const DYNAMIC_MODEL: u8 = 0;
    pub const STATIC_MODEL: u8 = 1;
}

impl Default for Object2270 {
    fn default() -> Self {
        Object2270 {
            id: 0,
            age: 0,
            prediction_age: 0,
            relative_moment_of_measurement: 0,
            reference_point_location: 0,
            reference_point_position_x: 0,
            reference_point_position_y: 0,
            reference_point_position_sigma_x: 0,
            reference_point_position_sigma_y: 0,
            contour_points_cog_x: 0,
            contour_points_cog_y: 0,
            object_box_length: 0,
            object_box_width: 0,
            object_box_orientation_angle: 0,
            object_box_orientation_angle_sigma: 0,
            absolute_velocity_x: 0,
            absolute_velocity_y: 0,
            absolute_velocity_sigma_x: 0,
            absolute_velocity_sigma_y: 0,
            relative_velocity_x: 0,
            relative_velocity_y: 0,
            relative_velocity_sigma_x: 0,
            relative_velocity_sigma_y: 0,
            classification: 0,
            tracking_model: 0,
            mobile_detected: false,
            track_valid: false,
            classification_age: 0,
            classification_confidence: 0,
            number_of_contour_points: 0,
            contour_point_list: Vec::new(),
        }
    }
}

impl ros2_client::Message for Object2270 {}
