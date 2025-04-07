use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BehaviourTree {
    pub behaviours: Vec<crate::py_trees_ros_interfaces::msg::Behaviour>,
    pub changed: bool,
    pub blackboard_on_visited_path: Vec<crate::diagnostic_msgs::msg::KeyValue>,
    pub blackboard_activity: Vec<crate::py_trees_ros_interfaces::msg::ActivityItem>,
    pub statistics: crate::py_trees_ros_interfaces::msg::Statistics,
}

impl Default for BehaviourTree {
    fn default() -> Self {
        BehaviourTree {
            behaviours: Vec::new(),
            changed: false,
            blackboard_on_visited_path: Vec::new(),
            blackboard_activity: Vec::new(),
            statistics: crate::py_trees_ros_interfaces::msg::Statistics::default(),
        }
    }
}

impl ros2_client::Message for BehaviourTree {}
