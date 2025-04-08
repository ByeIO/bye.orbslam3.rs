use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JointConstraint {
    pub joint_name: ::std::string::String,
    pub position: f64,
    pub tolerance_above: f64,
    pub tolerance_below: f64,
    pub weight: f64,
}

impl Default for JointConstraint {
    fn default() -> Self {
        JointConstraint {
            joint_name: ::std::string::String::new(),
            position: 0.0,
            tolerance_above: 0.0,
            tolerance_below: 0.0,
            weight: 0.0,
        }
    }
}

impl ros2_client::Message for JointConstraint {}
