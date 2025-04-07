use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AftermarketLane {
    pub header: crate::std_msgs::msg::Header,
    pub lane_confidence_left: u8,
    pub ldw_available_left: bool,
    pub lane_type_left: u8,
    pub distance_to_left_lane: f32,
    pub lane_confidence_right: u8,
    pub ldw_available_right: bool,
    pub lane_type_right: u8,
    pub distance_to_right_lane: f32,
}

impl AftermarketLane {
    pub const LANE_CONFIDENCE_NONE: u8 = 0;
    pub const LANE_CONFIDENCE_LOW: u8 = 1;
    pub const LANE_CONFIDENCE_MED: u8 = 2;
    pub const LANE_CONFIDENCE_HIGH: u8 = 3;
    pub const LANE_TYPE_DASHED: u8 = 0;
    pub const LANE_TYPE_SOLID: u8 = 1;
    pub const LANE_TYPE_NONE: u8 = 2;
    pub const LANE_TYPE_ROAD_EDGE: u8 = 3;
    pub const LANE_TYPE_DOUBLE_LANE_MARK: u8 = 4;
    pub const LANE_TYPE_BOTTS_DOTS: u8 = 5;
    pub const LANE_TYPE_INVALID: u8 = 6;
}

impl Default for AftermarketLane {
    fn default() -> Self {
        AftermarketLane {
            header: crate::std_msgs::msg::Header::default(),
            lane_confidence_left: 0,
            ldw_available_left: false,
            lane_type_left: 0,
            distance_to_left_lane: 0.0,
            lane_confidence_right: 0,
            ldw_available_right: false,
            lane_type_right: 0,
            distance_to_right_lane: 0.0,
        }
    }
}

impl ros2_client::Message for AftermarketLane {}
