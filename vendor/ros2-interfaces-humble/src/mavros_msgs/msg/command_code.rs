use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandCode {

}

impl CommandCode {
    pub const AIRFRAME_CONFIGURATION: u16 = 2520;
    pub const ARM_AUTHORIZATION_REQUEST: u16 = 3001;
    pub const CAMERA_TRACK_POINT: u16 = 2004;
    pub const CAMERA_TRACK_RECTANGLE: u16 = 2005;
    pub const CAMERA_STOP_TRACKING: u16 = 2010;
    pub const CAN_FORWARD: u16 = 32000;
    pub const COMPONENT_ARM_DISARM: u16 = 400;
    pub const CONDITION_DELAY: u16 = 112;
    pub const CONDITION_CHANGE_ALT: u16 = 113;
    pub const CONDITION_DISTANCE: u16 = 114;
    pub const CONDITION_YAW: u16 = 115;
    pub const CONDITION_LAST: u16 = 159;
    pub const CONTROL_HIGH_LATENCY: u16 = 2600;
    pub const DO_FOLLOW: u16 = 32;
    pub const DO_FOLLOW_REPOSITION: u16 = 33;
    pub const DO_SET_MODE: u16 = 176;
    pub const DO_JUMP: u16 = 177;
    pub const DO_CHANGE_SPEED: u16 = 178;
    pub const DO_SET_HOME: u16 = 179;
    pub const DO_SET_PARAMETER: u16 = 180;
    pub const DO_SET_RELAY: u16 = 181;
    pub const DO_REPEAT_RELAY: u16 = 182;
    pub const DO_SET_SERVO: u16 = 183;
    pub const DO_REPEAT_SERVO: u16 = 184;
    pub const DO_FLIGHTTERMINATION: u16 = 185;
    pub const DO_CHANGE_ALTITUDE: u16 = 186;
    pub const DO_LAND_START: u16 = 189;
    pub const DO_RALLY_LAND: u16 = 190;
    pub const DO_GO_AROUND: u16 = 191;
    pub const DO_REPOSITION: u16 = 192;
    pub const DO_PAUSE_CONTINUE: u16 = 193;
    pub const DO_SET_REVERSE: u16 = 194;
    pub const DO_SET_ROI_LOCATION: u16 = 195;
    pub const DO_SET_ROI_WPNEXT_OFFSET: u16 = 196;
    pub const DO_SET_ROI_NONE: u16 = 197;
    pub const DO_SET_ROI_SYSID: u16 = 198;
    pub const DO_CONTROL_VIDEO: u16 = 200;
    pub const DO_SET_ROI: u16 = 201;
    pub const DO_DIGICAM_CONFIGURE: u16 = 202;
    pub const DO_DIGICAM_CONTROL: u16 = 203;
    pub const DO_MOUNT_CONFIGURE: u16 = 204;
    pub const DO_MOUNT_CONTROL: u16 = 205;
    pub const DO_SET_CAM_TRIGG_DIST: u16 = 206;
    pub const DO_FENCE_ENABLE: u16 = 207;
    pub const DO_PARACHUTE: u16 = 208;
    pub const DO_MOTOR_TEST: u16 = 209;
    pub const DO_INVERTED_FLIGHT: u16 = 210;
    pub const DO_GRIPPER: u16 = 211;
    pub const DO_AUTOTUNE_ENABLE: u16 = 212;
    pub const DO_SET_CAM_TRIGG_INTERVAL: u16 = 214;
    pub const DO_MOUNT_CONTROL_QUAT: u16 = 220;
    pub const DO_GUIDED_MASTER: u16 = 221;
    pub const DO_GUIDED_LIMITS: u16 = 222;
    pub const DO_ENGINE_CONTROL: u16 = 223;
    pub const DO_SET_MISSION_CURRENT: u16 = 224;
    pub const DO_LAST: u16 = 240;
    pub const DO_JUMP_TAG: u16 = 601;
    pub const DO_GIMBAL_MANAGER_PITCHYAW: u16 = 1000;
    pub const DO_GIMBAL_MANAGER_CONFIGURE: u16 = 1001;
    pub const DO_TRIGGER_CONTROL: u16 = 2003;
    pub const DO_VTOL_TRANSITION: u16 = 3000;
    pub const DO_ADSB_OUT_IDENT: u16 = 10001;
    pub const DO_WINCH: u16 = 42600;
    pub const FIXED_MAG_CAL_YAW: u16 = 42006;
    pub const GET_HOME_POSITION: u16 = 410;
    pub const GET_MESSAGE_INTERVAL: u16 = 510;
    pub const IMAGE_START_CAPTURE: u16 = 2000;
    pub const IMAGE_STOP_CAPTURE: u16 = 2001;
    pub const JUMP_TAG: u16 = 600;
    pub const LOGGING_START: u16 = 2510;
    pub const LOGGING_STOP: u16 = 2511;
    pub const MISSION_START: u16 = 300;
    pub const NAV_WAYPOINT: u16 = 16;
    pub const NAV_LOITER_UNLIM: u16 = 17;
    pub const NAV_LOITER_TURNS: u16 = 18;
    pub const NAV_LOITER_TIME: u16 = 19;
    pub const NAV_RETURN_TO_LAUNCH: u16 = 20;
    pub const NAV_LAND: u16 = 21;
    pub const NAV_TAKEOFF: u16 = 22;
    pub const NAV_LAND_LOCAL: u16 = 23;
    pub const NAV_TAKEOFF_LOCAL: u16 = 24;
    pub const NAV_FOLLOW: u16 = 25;
    pub const NAV_CONTINUE_AND_CHANGE_ALT: u16 = 30;
    pub const NAV_LOITER_TO_ALT: u16 = 31;
    pub const NAV_ROI: u16 = 80;
    pub const NAV_PATHPLANNING: u16 = 81;
    pub const NAV_SPLINE_WAYPOINT: u16 = 82;
    pub const NAV_VTOL_TAKEOFF: u16 = 84;
    pub const NAV_VTOL_LAND: u16 = 85;
    pub const NAV_GUIDED_ENABLE: u16 = 92;
    pub const NAV_DELAY: u16 = 93;
    pub const NAV_PAYLOAD_PLACE: u16 = 94;
    pub const NAV_LAST: u16 = 95;
    pub const NAV_SET_YAW_SPEED: u16 = 213;
    pub const NAV_FENCE_RETURN_POINT: u16 = 5000;
    pub const NAV_FENCE_POLYGON_VERTEX_INCLUSION: u16 = 5001;
    pub const NAV_FENCE_POLYGON_VERTEX_EXCLUSION: u16 = 5002;
    pub const NAV_FENCE_CIRCLE_INCLUSION: u16 = 5003;
    pub const NAV_FENCE_CIRCLE_EXCLUSION: u16 = 5004;
    pub const NAV_RALLY_POINT: u16 = 5100;
    pub const OBLIQUE_SURVEY: u16 = 260;
    pub const OVERRIDE_GOTO: u16 = 252;
    pub const PANORAMA_CREATE: u16 = 2800;
    pub const PAYLOAD_PREPARE_DEPLOY: u16 = 30001;
    pub const PAYLOAD_CONTROL_DEPLOY: u16 = 30002;
    pub const PREFLIGHT_CALIBRATION: u16 = 241;
    pub const PREFLIGHT_SET_SENSOR_OFFSETS: u16 = 242;
    pub const PREFLIGHT_UAVCAN: u16 = 243;
    pub const PREFLIGHT_STORAGE: u16 = 245;
    pub const PREFLIGHT_REBOOT_SHUTDOWN: u16 = 246;
    pub const REQUEST_MESSAGE: u16 = 512;
    pub const REQUEST_PROTOCOL_VERSION: u16 = 519;
    pub const REQUEST_AUTOPILOT_CAPABILITIES: u16 = 520;
    pub const REQUEST_CAMERA_INFORMATION: u16 = 521;
    pub const REQUEST_CAMERA_SETTINGS: u16 = 522;
    pub const REQUEST_STORAGE_INFORMATION: u16 = 525;
    pub const REQUEST_CAMERA_CAPTURE_STATUS: u16 = 527;
    pub const REQUEST_FLIGHT_INFORMATION: u16 = 528;
    pub const REQUEST_VIDEO_STREAM_INFORMATION: u16 = 2504;
    pub const REQUEST_VIDEO_STREAM_STATUS: u16 = 2505;
    pub const RESET_CAMERA_SETTINGS: u16 = 529;
    pub const RUN_PREARM_CHECKS: u16 = 401;
    pub const SET_MESSAGE_INTERVAL: u16 = 511;
    pub const SET_CAMERA_MODE: u16 = 530;
    pub const SET_CAMERA_ZOOM: u16 = 531;
    pub const SET_CAMERA_FOCUS: u16 = 532;
    pub const SET_GUIDED_SUBMODE_STANDARD: u16 = 4000;
    pub const SET_GUIDED_SUBMODE_CIRCLE: u16 = 4001;
    pub const START_RX_PAIR: u16 = 500;
    pub const STORAGE_FORMAT: u16 = 526;
    pub const UAVCAN_GET_NODE_INFO: u16 = 5200;
    pub const VIDEO_START_CAPTURE: u16 = 2500;
    pub const VIDEO_STOP_CAPTURE: u16 = 2501;
    pub const VIDEO_START_STREAMING: u16 = 2502;
    pub const VIDEO_STOP_STREAMING: u16 = 2503;
}

impl Default for CommandCode {
    fn default() -> Self {
        CommandCode {

        }
    }
}

impl ros2_client::Message for CommandCode {}
