use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct State {
    pub header: crate::std_msgs::msg::Header,
    pub connected: bool,
    pub armed: bool,
    pub guided: bool,
    pub manual_input: bool,
    pub mode: ::std::string::String,
    pub system_status: u8,
}

impl State {
    pub const MODE_APM_PLANE_MANUAL: &'static str = "MANUAL";
    pub const MODE_APM_PLANE_CIRCLE: &'static str = "CIRCLE";
    pub const MODE_APM_PLANE_STABILIZE: &'static str = "STABILIZE";
    pub const MODE_APM_PLANE_TRAINING: &'static str = "TRAINING";
    pub const MODE_APM_PLANE_ACRO: &'static str = "ACRO";
    pub const MODE_APM_PLANE_FBWA: &'static str = "FBWA";
    pub const MODE_APM_PLANE_FBWB: &'static str = "FBWB";
    pub const MODE_APM_PLANE_CRUISE: &'static str = "CRUISE";
    pub const MODE_APM_PLANE_AUTOTUNE: &'static str = "AUTOTUNE";
    pub const MODE_APM_PLANE_AUTO: &'static str = "AUTO";
    pub const MODE_APM_PLANE_RTL: &'static str = "RTL";
    pub const MODE_APM_PLANE_LOITER: &'static str = "LOITER";
    pub const MODE_APM_PLANE_LAND: &'static str = "LAND";
    pub const MODE_APM_PLANE_GUIDED: &'static str = "GUIDED";
    pub const MODE_APM_PLANE_INITIALISING: &'static str = "INITIALISING";
    pub const MODE_APM_PLANE_QSTABILIZE: &'static str = "QSTABILIZE";
    pub const MODE_APM_PLANE_QHOVER: &'static str = "QHOVER";
    pub const MODE_APM_PLANE_QLOITER: &'static str = "QLOITER";
    pub const MODE_APM_PLANE_QLAND: &'static str = "QLAND";
    pub const MODE_APM_PLANE_QRTL: &'static str = "QRTL";
    pub const MODE_APM_COPTER_STABILIZE: &'static str = "STABILIZE";
    pub const MODE_APM_COPTER_ACRO: &'static str = "ACRO";
    pub const MODE_APM_COPTER_ALT_HOLD: &'static str = "ALT_HOLD";
    pub const MODE_APM_COPTER_AUTO: &'static str = "AUTO";
    pub const MODE_APM_COPTER_GUIDED: &'static str = "GUIDED";
    pub const MODE_APM_COPTER_LOITER: &'static str = "LOITER";
    pub const MODE_APM_COPTER_RTL: &'static str = "RTL";
    pub const MODE_APM_COPTER_CIRCLE: &'static str = "CIRCLE";
    pub const MODE_APM_COPTER_POSITION: &'static str = "POSITION";
    pub const MODE_APM_COPTER_LAND: &'static str = "LAND";
    pub const MODE_APM_COPTER_OF_LOITER: &'static str = "OF_LOITER";
    pub const MODE_APM_COPTER_DRIFT: &'static str = "DRIFT";
    pub const MODE_APM_COPTER_SPORT: &'static str = "SPORT";
    pub const MODE_APM_COPTER_FLIP: &'static str = "FLIP";
    pub const MODE_APM_COPTER_AUTOTUNE: &'static str = "AUTOTUNE";
    pub const MODE_APM_COPTER_POSHOLD: &'static str = "POSHOLD";
    pub const MODE_APM_COPTER_BRAKE: &'static str = "BRAKE";
    pub const MODE_APM_COPTER_THROW: &'static str = "THROW";
    pub const MODE_APM_COPTER_AVOID_ADSB: &'static str = "AVOID_ADSB";
    pub const MODE_APM_COPTER_GUIDED_NOGPS: &'static str = "GUIDED_NOGPS";
    pub const MODE_APM_ROVER_MANUAL: &'static str = "MANUAL";
    pub const MODE_APM_ROVER_LEARNING: &'static str = "LEARNING";
    pub const MODE_APM_ROVER_STEERING: &'static str = "STEERING";
    pub const MODE_APM_ROVER_HOLD: &'static str = "HOLD";
    pub const MODE_APM_ROVER_AUTO: &'static str = "AUTO";
    pub const MODE_APM_ROVER_RTL: &'static str = "RTL";
    pub const MODE_APM_ROVER_GUIDED: &'static str = "GUIDED";
    pub const MODE_APM_ROVER_INITIALISING: &'static str = "INITIALISING";
    pub const MODE_PX4_MANUAL: &'static str = "MANUAL";
    pub const MODE_PX4_ACRO: &'static str = "ACRO";
    pub const MODE_PX4_ALTITUDE: &'static str = "ALTCTL";
    pub const MODE_PX4_POSITION: &'static str = "POSCTL";
    pub const MODE_PX4_OFFBOARD: &'static str = "OFFBOARD";
    pub const MODE_PX4_STABILIZED: &'static str = "STABILIZED";
    pub const MODE_PX4_RATTITUDE: &'static str = "RATTITUDE";
    pub const MODE_PX4_MISSION: &'static str = "AUTO.MISSION";
    pub const MODE_PX4_LOITER: &'static str = "AUTO.LOITER";
    pub const MODE_PX4_RTL: &'static str = "AUTO.RTL";
    pub const MODE_PX4_LAND: &'static str = "AUTO.LAND";
    pub const MODE_PX4_RTGS: &'static str = "AUTO.RTGS";
    pub const MODE_PX4_READY: &'static str = "AUTO.READY";
    pub const MODE_PX4_TAKEOFF: &'static str = "AUTO.TAKEOFF";
}

impl Default for State {
    fn default() -> Self {
        State {
            header: crate::std_msgs::msg::Header::default(),
            connected: false,
            armed: false,
            guided: false,
            manual_input: false,
            mode: ::std::string::String::new(),
            system_status: 0,
        }
    }
}

impl ros2_client::Message for State {}
