use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Spline {
    pub header: crate::std_msgs::msg::Header,
    pub knots: Vec<f64>,
    pub ctrls: Vec<crate::tuw_nav_msgs::msg::Float64Array>,
}

impl Default for Spline {
    fn default() -> Self {
        Spline {
            header: crate::std_msgs::msg::Header::default(),
            knots: Vec::new(),
            ctrls: Vec::new(),
        }
    }
}

impl ros2_client::Message for Spline {}
