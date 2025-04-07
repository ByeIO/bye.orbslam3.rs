use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Route {
    pub map: ::std::string::String,
    pub trajectory: crate::rmf_traffic_msgs::msg::Trajectory,
    pub checkpoints: Vec<u64>,
    pub dependencies: Vec<crate::rmf_traffic_msgs::msg::TrafficDependency>,
}

impl Default for Route {
    fn default() -> Self {
        Route {
            map: ::std::string::String::new(),
            trajectory: crate::rmf_traffic_msgs::msg::Trajectory::default(),
            checkpoints: Vec::new(),
            dependencies: Vec::new(),
        }
    }
}

impl ros2_client::Message for Route {}
