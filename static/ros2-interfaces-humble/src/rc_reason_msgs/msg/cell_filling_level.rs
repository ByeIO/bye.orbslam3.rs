use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CellFillingLevel {
    pub cell_size: crate::rc_reason_msgs::msg::Rectangle,
    pub cell_position: crate::geometry_msgs::msg::Point,
    pub level_in_percent: crate::rc_reason_msgs::msg::RangeValue,
    pub level_free_in_meters: crate::rc_reason_msgs::msg::RangeValue,
    pub coverage: f64,
}

impl Default for CellFillingLevel {
    fn default() -> Self {
        CellFillingLevel {
            cell_size: crate::rc_reason_msgs::msg::Rectangle::default(),
            cell_position: crate::geometry_msgs::msg::Point::default(),
            level_in_percent: crate::rc_reason_msgs::msg::RangeValue::default(),
            level_free_in_meters: crate::rc_reason_msgs::msg::RangeValue::default(),
            coverage: 0.0,
        }
    }
}

impl ros2_client::Message for CellFillingLevel {}
