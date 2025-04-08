use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetForceModeRequest {
    pub task_frame: crate::geometry_msgs::msg::PoseStamped,
    pub selection_vector_x: bool, // default: 0
    pub selection_vector_y: bool, // default: 0
    pub selection_vector_z: bool, // default: 0
    pub selection_vector_rx: bool, // default: 0
    pub selection_vector_ry: bool, // default: 0
    pub selection_vector_rz: bool, // default: 0
    pub wrench: crate::geometry_msgs::msg::Wrench,
    #[serde(rename = "type")]    pub type_: u8, // default: 2
    pub speed_limits: crate::geometry_msgs::msg::Twist,
    pub deviation_limits: [f32; 6], // default: [0.01, 0.01, 0.01, 0.01, 0.01, 0.01]
    pub damping_factor: f32, // default: 0.025
    pub gain_scaling: f32, // default: 0.5
}

impl SetForceModeRequest {
    pub const TCP_TO_ORIGIN: u8 = 1;
    pub const NO_TRANSFORM: u8 = 2;
    pub const TCP_VELOCITY_TO_X_Y: u8 = 3;
}

impl Default for SetForceModeRequest {
    fn default() -> Self {
        SetForceModeRequest {
            task_frame: crate::geometry_msgs::msg::PoseStamped::default(),
            selection_vector_x: false,
            selection_vector_y: false,
            selection_vector_z: false,
            selection_vector_rx: false,
            selection_vector_ry: false,
            selection_vector_rz: false,
            wrench: crate::geometry_msgs::msg::Wrench::default(),
            type_: 2,
            speed_limits: crate::geometry_msgs::msg::Twist::default(),
            deviation_limits: [0.01, 0.01, 0.01, 0.01, 0.01, 0.01],
            damping_factor: 0.025,
            gain_scaling: 0.5,
        }
    }
}

impl ros2_client::Message for SetForceModeRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetForceModeResponse {
    pub success: bool,
}

impl Default for SetForceModeResponse {
    fn default() -> Self {
        SetForceModeResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for SetForceModeResponse {}


pub struct SetForceMode;
impl ros2_client::Service for SetForceMode {
    type Request = SetForceModeRequest;
    type Response = SetForceModeResponse;

    fn request_type_name(&self) -> &str { "SetForceModeRequest" }
    fn response_type_name(&self) -> &str { "SetForceModeResponse" }
}
