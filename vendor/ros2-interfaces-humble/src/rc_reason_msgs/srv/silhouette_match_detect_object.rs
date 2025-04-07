use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SilhouetteMatchDetectObjectRequest {
    pub object_to_detect: crate::rc_reason_msgs::msg::SilhouetteMatchObject,
    pub offset: f64,
    pub pose_frame: ::std::string::String,
    pub robot_pose: crate::geometry_msgs::msg::Pose,
    pub load_carrier_id: ::std::string::String,
    pub collision_detection: crate::rc_reason_msgs::msg::CollisionDetection,
    pub object_plane_detection: bool,
}

impl Default for SilhouetteMatchDetectObjectRequest {
    fn default() -> Self {
        SilhouetteMatchDetectObjectRequest {
            object_to_detect: crate::rc_reason_msgs::msg::SilhouetteMatchObject::default(),
            offset: 0.0,
            pose_frame: ::std::string::String::new(),
            robot_pose: crate::geometry_msgs::msg::Pose::default(),
            load_carrier_id: ::std::string::String::new(),
            collision_detection: crate::rc_reason_msgs::msg::CollisionDetection::default(),
            object_plane_detection: false,
        }
    }
}

impl ros2_client::Message for SilhouetteMatchDetectObjectRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SilhouetteMatchDetectObjectResponse {
    pub timestamp: crate::builtin_interfaces::msg::Time,
    pub object_id: ::std::string::String,
    pub matches: Vec<crate::rc_reason_msgs::msg::Match>,
    pub grasps: Vec<crate::rc_reason_msgs::msg::Grasp>,
    pub load_carriers: Vec<crate::rc_reason_msgs::msg::LoadCarrier>,
    pub return_code: crate::rc_common_msgs::msg::ReturnCode,
}

impl Default for SilhouetteMatchDetectObjectResponse {
    fn default() -> Self {
        SilhouetteMatchDetectObjectResponse {
            timestamp: crate::builtin_interfaces::msg::Time::default(),
            object_id: ::std::string::String::new(),
            matches: Vec::new(),
            grasps: Vec::new(),
            load_carriers: Vec::new(),
            return_code: crate::rc_common_msgs::msg::ReturnCode::default(),
        }
    }
}

impl ros2_client::Message for SilhouetteMatchDetectObjectResponse {}


pub struct SilhouetteMatchDetectObject;
impl ros2_client::Service for SilhouetteMatchDetectObject {
    type Request = SilhouetteMatchDetectObjectRequest;
    type Response = SilhouetteMatchDetectObjectResponse;

    fn request_type_name(&self) -> &str { "SilhouetteMatchDetectObjectRequest" }
    fn response_type_name(&self) -> &str { "SilhouetteMatchDetectObjectResponse" }
}
