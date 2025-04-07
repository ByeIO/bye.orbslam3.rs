use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UntrackedProperties {
    pub relative_time_of_measurement: u16,
    pub position_closest_point: crate::ibeo_msgs::msg::Point2Di,
    pub object_box_size: crate::ibeo_msgs::msg::Point2Di,
    pub object_box_size_sigma: crate::ibeo_msgs::msg::Point2Dui,
    pub object_box_orientation: i16,
    pub object_box_orientation_sigma: u16,
    pub tracking_point_coordinate: crate::ibeo_msgs::msg::Point2Di,
    pub tracking_point_coordinate_sigma: crate::ibeo_msgs::msg::Point2Dui,
    pub number_of_contour_points: u8,
    pub contour_point_list: Vec<crate::ibeo_msgs::msg::ContourPointSigma>,
}

impl Default for UntrackedProperties {
    fn default() -> Self {
        UntrackedProperties {
            relative_time_of_measurement: 0,
            position_closest_point: crate::ibeo_msgs::msg::Point2Di::default(),
            object_box_size: crate::ibeo_msgs::msg::Point2Di::default(),
            object_box_size_sigma: crate::ibeo_msgs::msg::Point2Dui::default(),
            object_box_orientation: 0,
            object_box_orientation_sigma: 0,
            tracking_point_coordinate: crate::ibeo_msgs::msg::Point2Di::default(),
            tracking_point_coordinate_sigma: crate::ibeo_msgs::msg::Point2Dui::default(),
            number_of_contour_points: 0,
            contour_point_list: Vec::new(),
        }
    }
}

impl ros2_client::Message for UntrackedProperties {}
