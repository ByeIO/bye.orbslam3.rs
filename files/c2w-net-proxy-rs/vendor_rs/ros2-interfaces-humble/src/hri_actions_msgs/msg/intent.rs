use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Intent {
    pub intent: ::std::string::String,
    pub data: ::std::string::String,
    pub source: ::std::string::String,
    pub modality: ::std::string::String,
    pub priority: u8,
    pub confidence: f32,
}

impl Intent {
    pub const RAW_USER_INPUT: &'static str = "__raw_user_input__";
    pub const ENGAGE_WITH: &'static str = "__intent_engage_with__";
    pub const MOVE_TO: &'static str = "__intent_move_to__";
    pub const GUIDE: &'static str = "__intent_guide__";
    pub const GRAB_OBJECT: &'static str = "__intent_grab_object__";
    pub const BRING_OBJECT: &'static str = "__intent_bring_object__";
    pub const PLACE_OBJECT: &'static str = "__intent_place_object__";
    pub const GREET: &'static str = "__intent_greet__";
    pub const SAY: &'static str = "__intent_say__";
    pub const PRESENT_CONTENT: &'static str = "__intent_present_content__";
    pub const PERFORM_MOTION: &'static str = "__intent_perform_motion__";
    pub const START_ACTIVITY: &'static str = "__intent_start_activity__";
    pub const STOP_ACTIVITY: &'static str = "__intent_stop_activity__";
    pub const WAKEUP: &'static str = "__intent_wakeup__";
    pub const SUSPEND: &'static str = "__intent_suspend__";
    pub const ROBOT_ITSELF: &'static str = "__myself__";
    pub const REMOTE_SUPERVISOR: &'static str = "__remote_supervisor__";
    pub const UNKNOWN_AGENT: &'static str = "__unknown_agent__";
    pub const UNKNOWN: &'static str = "__unknown__";
    pub const MODALITY_SPEECH: &'static str = "__modality_speech__";
    pub const MODALITY_MOTION: &'static str = "__modality_motion__";
    pub const MODALITY_TOUCHSCREEN: &'static str = "__modality_touchscreen__";
    pub const MODALITY_OTHER: &'static str = "__modality_other__";
    pub const MODALITY_INTERNAL: &'static str = "__modality_internal__";
}

impl Default for Intent {
    fn default() -> Self {
        Intent {
            intent: ::std::string::String::new(),
            data: ::std::string::String::new(),
            source: ::std::string::String::new(),
            modality: ::std::string::String::new(),
            priority: 0,
            confidence: 0.0,
        }
    }
}

impl ros2_client::Message for Intent {}
