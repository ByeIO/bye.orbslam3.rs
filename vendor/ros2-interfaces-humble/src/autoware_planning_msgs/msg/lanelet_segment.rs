use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LaneletSegment {
    pub preferred_primitive: crate::autoware_planning_msgs::msg::LaneletPrimitive,
    pub primitives: Vec<crate::autoware_planning_msgs::msg::LaneletPrimitive>,
}

impl Default for LaneletSegment {
    fn default() -> Self {
        LaneletSegment {
            preferred_primitive: crate::autoware_planning_msgs::msg::LaneletPrimitive::default(),
            primitives: Vec::new(),
        }
    }
}

impl ros2_client::Message for LaneletSegment {}
