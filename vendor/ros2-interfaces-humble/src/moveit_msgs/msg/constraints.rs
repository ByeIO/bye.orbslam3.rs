use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Constraints {
    pub name: ::std::string::String,
    pub joint_constraints: Vec<crate::moveit_msgs::msg::JointConstraint>,
    pub position_constraints: Vec<crate::moveit_msgs::msg::PositionConstraint>,
    pub orientation_constraints: Vec<crate::moveit_msgs::msg::OrientationConstraint>,
    pub visibility_constraints: Vec<crate::moveit_msgs::msg::VisibilityConstraint>,
}

impl Default for Constraints {
    fn default() -> Self {
        Constraints {
            name: ::std::string::String::new(),
            joint_constraints: Vec::new(),
            position_constraints: Vec::new(),
            orientation_constraints: Vec::new(),
            visibility_constraints: Vec::new(),
        }
    }
}

impl ros2_client::Message for Constraints {}
