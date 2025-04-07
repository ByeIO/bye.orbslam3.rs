use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Lane {
    pub quality: u8,
    pub marker_kind: u8,
    pub curve_model_kind: u8,
    pub marker_offset: f64,
    pub heading_angle: f64,
    pub curvature: f64,
    pub curvature_derivative: f64,
    pub marker_width: f64,
    pub view_range: f64,
}

impl Lane {
    pub const LANE_QUALITY_INVALID: u8 = 0;
    pub const LANE_QUALITY_UNKNOWN: u8 = 1;
    pub const LANE_QUALITY_NOT_AVAILABLE: u8 = 2;
    pub const LANE_QUALITY_0: u8 = 3;
    pub const LANE_QUALITY_1: u8 = 4;
    pub const LANE_QUALITY_2: u8 = 5;
    pub const LANE_QUALITY_3: u8 = 6;
    pub const LANE_QUALITY_4: u8 = 7;
    pub const LANE_QUALITY_5: u8 = 8;
    pub const LANE_QUALITY_6: u8 = 9;
    pub const LANE_QUALITY_7: u8 = 10;
    pub const LANE_QUALITY_8: u8 = 11;
    pub const LANE_QUALITY_9: u8 = 12;
    pub const LANE_QUALITY_KIND_COUNT: u8 = 13;
    pub const LANE_MARKER_INVALID: u8 = 0;
    pub const LANE_MARKER_UNKNOWN: u8 = 1;
    pub const LANE_MARKER_NOT_AVAILABLE: u8 = 2;
    pub const LANE_MARKER_NONE: u8 = 3;
    pub const LANE_MARKER_SOLID: u8 = 4;
    pub const LANE_MARKER_DASHED: u8 = 5;
    pub const LANE_MARKER_VIRTUAL: u8 = 6;
    pub const LANE_MARKER_DOTS: u8 = 7;
    pub const LANE_MARKER_ROAD_EDGE: u8 = 8;
    pub const LANE_MARKER_DOUBLE_LINE: u8 = 9;
    pub const LANE_MARKER_KIND_COUNT: u8 = 10;
    pub const LANE_CURVE_MODEL_INVALID: u8 = 0;
    pub const LANE_CURVE_MODEL_UNKNOWN: u8 = 1;
    pub const LANE_CURVE_MODEL_NOT_AVAILABLE: u8 = 2;
    pub const LANE_CURVE_MODEL_LINEAR: u8 = 3;
    pub const LANE_CURVE_MODEL_PARABOLIC: u8 = 4;
    pub const LANE_CURVE_MODEL_3D: u8 = 5;
    pub const LANE_CURVE_MODEL_KIND_COUNT: u8 = 6;
}

impl Default for Lane {
    fn default() -> Self {
        Lane {
            quality: 0,
            marker_kind: 0,
            curve_model_kind: 0,
            marker_offset: 0.0,
            heading_angle: 0.0,
            curvature: 0.0,
            curvature_derivative: 0.0,
            marker_width: 0.0,
            view_range: 0.0,
        }
    }
}

impl ros2_client::Message for Lane {}
