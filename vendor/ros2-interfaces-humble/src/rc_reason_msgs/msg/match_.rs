use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Match {
    pub template_id: ::std::string::String,
    pub uuid: ::std::string::String,
    pub pose: crate::geometry_msgs::msg::PoseStamped,
    pub grasp_uuids: Vec<::std::string::String>,
    pub score: f64,
}

impl Default for Match {
    fn default() -> Self {
        Match {
            template_id: ::std::string::String::new(),
            uuid: ::std::string::String::new(),
            pose: crate::geometry_msgs::msg::PoseStamped::default(),
            grasp_uuids: Vec::new(),
            score: 0.0,
        }
    }
}

impl ros2_client::Message for Match {}
