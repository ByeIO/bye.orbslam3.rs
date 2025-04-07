use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeatureModule {
    pub id: u8,
}

impl FeatureModule {
    pub const MODULE_REMOTE_DEVICE: u8 = 0;
    pub const MODULE_SYSTEM: u8 = 1;
    pub const MODULE_INTERFACE: u8 = 2;
    pub const MODULE_LOCAL_DEVICE: u8 = 3;
    pub const MODULE_STREAM: u8 = 4;
}

impl Default for FeatureModule {
    fn default() -> Self {
        FeatureModule {
            id: 0,
        }
    }
}

impl ros2_client::Message for FeatureModule {}
