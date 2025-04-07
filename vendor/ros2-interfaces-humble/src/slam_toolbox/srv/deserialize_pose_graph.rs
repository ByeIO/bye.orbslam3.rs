use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeserializePoseGraphRequest {
    pub filename: ::std::string::String,
    pub match_type: i8,
    pub initial_pose: crate::geometry_msgs::msg::Pose2D,
}

impl DeserializePoseGraphRequest {
    pub const UNSET: i8 = 0;
    pub const START_AT_FIRST_NODE: i8 = 1;
    pub const START_AT_GIVEN_POSE: i8 = 2;
    pub const LOCALIZE_AT_POSE: i8 = 3;
}

impl Default for DeserializePoseGraphRequest {
    fn default() -> Self {
        DeserializePoseGraphRequest {
            filename: ::std::string::String::new(),
            match_type: 0,
            initial_pose: crate::geometry_msgs::msg::Pose2D::default(),
        }
    }
}

impl ros2_client::Message for DeserializePoseGraphRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeserializePoseGraphResponse {

}

impl Default for DeserializePoseGraphResponse {
    fn default() -> Self {
        DeserializePoseGraphResponse {

        }
    }
}

impl ros2_client::Message for DeserializePoseGraphResponse {}


pub struct DeserializePoseGraph;
impl ros2_client::Service for DeserializePoseGraph {
    type Request = DeserializePoseGraphRequest;
    type Response = DeserializePoseGraphResponse;

    fn request_type_name(&self) -> &str { "DeserializePoseGraphRequest" }
    fn response_type_name(&self) -> &str { "DeserializePoseGraphResponse" }
}
