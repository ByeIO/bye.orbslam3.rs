use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LkaLane {
    pub header: crate::std_msgs::msg::Header,
    pub lane_type: u8,
    pub quality: u8,
    pub model_degree: u8,
    pub position_parameter_c0: f64,
    pub curvature_parameter_c2: f64,
    pub curvature_derivative_parameter_c3: f64,
    pub marking_width: f32,
    pub heading_angle_parameter_c1: f64,
    pub view_range: f32,
    pub view_range_availability: bool,
}

impl LkaLane {
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

impl Default for LkaLane {
    fn default() -> Self {
        LkaLane {
            header: crate::std_msgs::msg::Header::default(),
            lane_type: 0,
            quality: 0,
            model_degree: 0,
            position_parameter_c0: 0.0,
            curvature_parameter_c2: 0.0,
            curvature_derivative_parameter_c3: 0.0,
            marking_width: 0.0,
            heading_angle_parameter_c1: 0.0,
            view_range: 0.0,
            view_range_availability: false,
        }
    }
}

impl ros2_client::Message for LkaLane {}
