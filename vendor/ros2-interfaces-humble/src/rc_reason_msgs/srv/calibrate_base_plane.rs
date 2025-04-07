use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CalibrateBasePlaneRequest {
    pub pose_frame: ::std::string::String,
    pub robot_pose: crate::geometry_msgs::msg::Pose,
    pub plane_estimation_method: ::std::string::String,
    pub stereo_plane_preference: ::std::string::String,
    pub region_of_interest_2d_id: ::std::string::String,
    pub offset: f64,
    pub plane: crate::shape_msgs::msg::Plane,
}

impl Default for CalibrateBasePlaneRequest {
    fn default() -> Self {
        CalibrateBasePlaneRequest {
            pose_frame: ::std::string::String::new(),
            robot_pose: crate::geometry_msgs::msg::Pose::default(),
            plane_estimation_method: ::std::string::String::new(),
            stereo_plane_preference: ::std::string::String::new(),
            region_of_interest_2d_id: ::std::string::String::new(),
            offset: 0.0,
            plane: crate::shape_msgs::msg::Plane::default(),
        }
    }
}

impl ros2_client::Message for CalibrateBasePlaneRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CalibrateBasePlaneResponse {
    pub timestamp: crate::builtin_interfaces::msg::Time,
    pub pose_frame: ::std::string::String,
    pub plane: crate::shape_msgs::msg::Plane,
    pub return_code: crate::rc_common_msgs::msg::ReturnCode,
}

impl Default for CalibrateBasePlaneResponse {
    fn default() -> Self {
        CalibrateBasePlaneResponse {
            timestamp: crate::builtin_interfaces::msg::Time::default(),
            pose_frame: ::std::string::String::new(),
            plane: crate::shape_msgs::msg::Plane::default(),
            return_code: crate::rc_common_msgs::msg::ReturnCode::default(),
        }
    }
}

impl ros2_client::Message for CalibrateBasePlaneResponse {}


pub struct CalibrateBasePlane;
impl ros2_client::Service for CalibrateBasePlane {
    type Request = CalibrateBasePlaneRequest;
    type Response = CalibrateBasePlaneResponse;

    fn request_type_name(&self) -> &str { "CalibrateBasePlaneRequest" }
    fn response_type_name(&self) -> &str { "CalibrateBasePlaneResponse" }
}
