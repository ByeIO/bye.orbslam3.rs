use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Info {
    pub header: crate::std_msgs::msg::Header,
    pub ref_id: i32,
    pub loop_closure_id: i32,
    pub proximity_detection_id: i32,
    pub landmark_id: i32,
    pub loop_closure_transform: crate::geometry_msgs::msg::Transform,
    pub wm_state: Vec<i32>,
    pub posterior_keys: Vec<i32>,
    pub posterior_values: Vec<f32>,
    pub likelihood_keys: Vec<i32>,
    pub likelihood_values: Vec<f32>,
    pub raw_likelihood_keys: Vec<i32>,
    pub raw_likelihood_values: Vec<f32>,
    pub weights_keys: Vec<i32>,
    pub weights_values: Vec<i32>,
    pub labels_keys: Vec<i32>,
    pub labels_values: Vec<::std::string::String>,
    pub stats_keys: Vec<::std::string::String>,
    pub stats_values: Vec<f32>,
    pub local_path: Vec<i32>,
    pub current_goal_id: i32,
    pub odom_cache: crate::rtabmap_msgs::msg::MapGraph,
}

impl Default for Info {
    fn default() -> Self {
        Info {
            header: crate::std_msgs::msg::Header::default(),
            ref_id: 0,
            loop_closure_id: 0,
            proximity_detection_id: 0,
            landmark_id: 0,
            loop_closure_transform: crate::geometry_msgs::msg::Transform::default(),
            wm_state: Vec::new(),
            posterior_keys: Vec::new(),
            posterior_values: Vec::new(),
            likelihood_keys: Vec::new(),
            likelihood_values: Vec::new(),
            raw_likelihood_keys: Vec::new(),
            raw_likelihood_values: Vec::new(),
            weights_keys: Vec::new(),
            weights_values: Vec::new(),
            labels_keys: Vec::new(),
            labels_values: Vec::new(),
            stats_keys: Vec::new(),
            stats_values: Vec::new(),
            local_path: Vec::new(),
            current_goal_id: 0,
            odom_cache: crate::rtabmap_msgs::msg::MapGraph::default(),
        }
    }
}

impl ros2_client::Message for Info {}
