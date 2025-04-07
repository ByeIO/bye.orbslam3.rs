use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeatureFlags {
    pub flag_none: bool,
    pub flag_read: bool,
    pub flag_write: bool,
    pub flag_volatile: bool,
    pub flag_modify_write: bool,
}

impl Default for FeatureFlags {
    fn default() -> Self {
        FeatureFlags {
            flag_none: false,
            flag_read: false,
            flag_write: false,
            flag_volatile: false,
            flag_modify_write: false,
        }
    }
}

impl ros2_client::Message for FeatureFlags {}
