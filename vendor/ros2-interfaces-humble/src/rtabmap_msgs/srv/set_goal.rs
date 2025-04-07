use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetGoalRequest {
    pub node_id: i32,
    pub node_label: ::std::string::String,
    pub frame_id: ::std::string::String,
}

impl Default for SetGoalRequest {
    fn default() -> Self {
        SetGoalRequest {
            node_id: 0,
            node_label: ::std::string::String::new(),
            frame_id: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SetGoalRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetGoalResponse {
    pub path_ids: Vec<i32>,
    pub path_poses: Vec<crate::geometry_msgs::msg::Pose>,
    pub planning_time: f32,
}

impl Default for SetGoalResponse {
    fn default() -> Self {
        SetGoalResponse {
            path_ids: Vec::new(),
            path_poses: Vec::new(),
            planning_time: 0.0,
        }
    }
}

impl ros2_client::Message for SetGoalResponse {}


pub struct SetGoal;
impl ros2_client::Service for SetGoal {
    type Request = SetGoalRequest;
    type Response = SetGoalResponse;

    fn request_type_name(&self) -> &str { "SetGoalRequest" }
    fn response_type_name(&self) -> &str { "SetGoalResponse" }
}
