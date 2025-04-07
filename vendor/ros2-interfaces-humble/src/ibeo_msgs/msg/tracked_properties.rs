use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrackedProperties {
    pub object_age: u16,
    pub hidden_status_age: u16,
    pub object_phase: u8,
    pub dynamic_property: u8,
    pub relative_time_of_measure: u16,
    pub position_closest_point: crate::ibeo_msgs::msg::Point2Di,
    pub relative_velocity: crate::ibeo_msgs::msg::Point2Di,
    pub relative_velocity_sigma: crate::ibeo_msgs::msg::Point2Dui,
    pub classification: u8,
    pub classification_age: u16,
    pub object_box_size: crate::ibeo_msgs::msg::Point2Di,
    pub object_box_size_sigma: crate::ibeo_msgs::msg::Point2Dui,
    pub object_box_orientation: i16,
    pub object_box_orientation_sigma: u16,
    pub tracking_point_location: u8,
    pub tracking_point_coordinate: crate::ibeo_msgs::msg::Point2Di,
    pub tracking_point_coordinate_sigma: crate::ibeo_msgs::msg::Point2Dui,
    pub velocity: crate::ibeo_msgs::msg::Point2Di,
    pub velocity_sigma: crate::ibeo_msgs::msg::Point2Dui,
    pub acceleration: crate::ibeo_msgs::msg::Point2Di,
    pub acceleration_sigma: crate::ibeo_msgs::msg::Point2Dui,
    pub yaw_rate: i16,
    pub yaw_rate_sigma: u16,
    pub number_of_contour_points: u8,
    pub contour_point_list: Vec<crate::ibeo_msgs::msg::ContourPointSigma>,
}

impl TrackedProperties {
    pub const INITIALIZATION_PHASE: u8 = 0;
    pub const TRACKING_PHASE: u8 = 1;
    pub const DYNAMIC_AND_MOVING: u8 = 0;
    pub const DYNAMIC_AND_STOPPED: u8 = 1;
    pub const A_PRIORI_STATIONARY: u8 = 2;
    pub const UNCLASSIFIED: u8 = 0;
    pub const UNKNOWN_SMALL: u8 = 1;
    pub const UNKNOWN_BIG: u8 = 2;
    pub const PEDESTRIAN: u8 = 3;
    pub const BIKE: u8 = 4;
    pub const CAR: u8 = 5;
    pub const TRUCK: u8 = 6;
    pub const OVER_DRIVABLE: u8 = 10;
    pub const UNDER_DRIVABLE: u8 = 11;
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
}

impl Default for TrackedProperties {
    fn default() -> Self {
        TrackedProperties {
            object_age: 0,
            hidden_status_age: 0,
            object_phase: 0,
            dynamic_property: 0,
            relative_time_of_measure: 0,
            position_closest_point: crate::ibeo_msgs::msg::Point2Di::default(),
            relative_velocity: crate::ibeo_msgs::msg::Point2Di::default(),
            relative_velocity_sigma: crate::ibeo_msgs::msg::Point2Dui::default(),
            classification: 0,
            classification_age: 0,
            object_box_size: crate::ibeo_msgs::msg::Point2Di::default(),
            object_box_size_sigma: crate::ibeo_msgs::msg::Point2Dui::default(),
            object_box_orientation: 0,
            object_box_orientation_sigma: 0,
            tracking_point_location: 0,
            tracking_point_coordinate: crate::ibeo_msgs::msg::Point2Di::default(),
            tracking_point_coordinate_sigma: crate::ibeo_msgs::msg::Point2Dui::default(),
            velocity: crate::ibeo_msgs::msg::Point2Di::default(),
            velocity_sigma: crate::ibeo_msgs::msg::Point2Dui::default(),
            acceleration: crate::ibeo_msgs::msg::Point2Di::default(),
            acceleration_sigma: crate::ibeo_msgs::msg::Point2Dui::default(),
            yaw_rate: 0,
            yaw_rate_sigma: 0,
            number_of_contour_points: 0,
            contour_point_list: Vec::new(),
        }
    }
}

impl ros2_client::Message for TrackedProperties {}
