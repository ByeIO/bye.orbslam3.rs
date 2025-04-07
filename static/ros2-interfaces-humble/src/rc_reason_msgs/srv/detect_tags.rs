use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DetectTagsRequest {
    pub tags: Vec<crate::rc_reason_msgs::msg::Tag>,
    pub pose_frame: ::std::string::String,
    pub robot_pose: crate::geometry_msgs::msg::Pose,
}

impl Default for DetectTagsRequest {
    fn default() -> Self {
        DetectTagsRequest {
            tags: Vec::new(),
            pose_frame: ::std::string::String::new(),
            robot_pose: crate::geometry_msgs::msg::Pose::default(),
        }
    }
}

impl ros2_client::Message for DetectTagsRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DetectTagsResponse {
    pub tags: Vec<crate::rc_reason_msgs::msg::DetectedTag>,
    pub timestamp: crate::builtin_interfaces::msg::Time,
    pub return_code: crate::rc_common_msgs::msg::ReturnCode,
}

impl Default for DetectTagsResponse {
    fn default() -> Self {
        DetectTagsResponse {
            tags: Vec::new(),
            timestamp: crate::builtin_interfaces::msg::Time::default(),
            return_code: crate::rc_common_msgs::msg::ReturnCode::default(),
        }
    }
}

impl ros2_client::Message for DetectTagsResponse {}


pub struct DetectTags;
impl ros2_client::Service for DetectTags {
    type Request = DetectTagsRequest;
    type Response = DetectTagsResponse;

    fn request_type_name(&self) -> &str { "DetectTagsRequest" }
    fn response_type_name(&self) -> &str { "DetectTagsResponse" }
}
