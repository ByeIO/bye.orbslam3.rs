use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObstacleData {
    pub header: crate::std_msgs::msg::Header,
    pub obstacle_id: u16,
    pub obstacle_pos_x: f64,
    pub obstacle_pos_y: f64,
    pub blinker_info: u8,
    pub cut_in_and_out: u8,
    pub obstacle_rel_vel_x: f64,
    pub obstacle_type: u8,
    pub obstacle_status: u8,
    pub obstacle_brake_lights: bool,
    pub obstacle_valid: u8,
    pub obstacle_length: f32,
    pub obstacle_width: f32,
    pub obstacle_age: u16,
    pub obstacle_lane: u8,
    pub cipv_flag: bool,
    pub radar_pos_x: f32,
    pub radar_vel_x: f32,
    pub radar_match_confidence: u8,
    pub matched_radar_id: u16,
    pub obstacle_angle_rate: f32,
    pub obstacle_scale_change: f64,
    pub object_accel_x: f32,
    pub obstacle_replaced: bool,
    pub obstacle_angle: f32,
}

impl ObstacleData {
    pub const BLINKER_INFO_UNAVAILABLE: u8 = 0;
    pub const BLINKER_INFO_OFF: u8 = 1;
    pub const BLINKER_INFO_LEFT: u8 = 2;
    pub const BLINKER_INFO_RIGHT: u8 = 3;
    pub const BLINKER_INFO_BOTH: u8 = 4;
    pub const CUT_IN_AND_OUT_UNDEFINED: u8 = 0;
    pub const CUT_IN_AND_OUT_IN_HOST_LANE: u8 = 1;
    pub const CUT_IN_AND_OUT_OUT_HOST_LANE: u8 = 2;
    pub const CUT_IN_AND_OUT_CUT_IN: u8 = 3;
    pub const CUT_IN_AND_OUT_CUT_OUT: u8 = 4;
    pub const OBSTACLE_TYPE_VEHICLE: u8 = 0;
    pub const OBSTACLE_TYPE_TRUCK: u8 = 1;
    pub const OBSTACLE_TYPE_BIKE: u8 = 2;
    pub const OBSTACLE_TYPE_PED: u8 = 3;
    pub const OBSTACLE_TYPE_BICYCLE: u8 = 4;
    pub const OBSTACLE_STATUS_UNDEFINED: u8 = 0;
    pub const OBSTACLE_STATUS_STANDING: u8 = 1;
    pub const OBSTACLE_STATUS_STOPPED: u8 = 2;
    pub const OBSTACLE_STATUS_MOVING: u8 = 3;
    pub const OBSTACLE_STATUS_ONCOMING: u8 = 4;
    pub const OBSTACLE_STATUS_PARKED: u8 = 5;
    pub const OBSTACLE_VALID_INVALID: u8 = 0;
    pub const OBSTACLE_VALID_NEW: u8 = 1;
    pub const OBSTACLE_VALID_OLDER: u8 = 2;
    pub const OBSTACLE_LANE_NOT_ASSIGNED: u8 = 0;
    pub const OBSTACLE_LANE_EGO_LANE: u8 = 1;
    pub const OBSTACLE_LANE_NEXT_LANE: u8 = 2;
    pub const OBSTACLE_LANE_INVALID: u8 = 3;
    pub const RADAR_MATCH_CONFIDENCE_NO_MATCH: u8 = 0;
    pub const RADAR_MATCH_CONFIDENCE_MULTI_MATCH: u8 = 1;
    pub const RADAR_MATCH_CONFIDENCE_BOUNDED_LOW: u8 = 2;
    pub const RADAR_MATCH_CONFIDENCE_BOUNDED_MED: u8 = 3;
    pub const RADAR_MATCH_CONFIDENCE_BOUNDED_HIGH: u8 = 4;
    pub const RADAR_MATCH_CONFIDENCE_HIGH: u8 = 5;
}

impl Default for ObstacleData {
    fn default() -> Self {
        ObstacleData {
            header: crate::std_msgs::msg::Header::default(),
            obstacle_id: 0,
            obstacle_pos_x: 0.0,
            obstacle_pos_y: 0.0,
            blinker_info: 0,
            cut_in_and_out: 0,
            obstacle_rel_vel_x: 0.0,
            obstacle_type: 0,
            obstacle_status: 0,
            obstacle_brake_lights: false,
            obstacle_valid: 0,
            obstacle_length: 0.0,
            obstacle_width: 0.0,
            obstacle_age: 0,
            obstacle_lane: 0,
            cipv_flag: false,
            radar_pos_x: 0.0,
            radar_vel_x: 0.0,
            radar_match_confidence: 0,
            matched_radar_id: 0,
            obstacle_angle_rate: 0.0,
            obstacle_scale_change: 0.0,
            object_accel_x: 0.0,
            obstacle_replaced: false,
            obstacle_angle: 0.0,
        }
    }
}

impl ros2_client::Message for ObstacleData {}
