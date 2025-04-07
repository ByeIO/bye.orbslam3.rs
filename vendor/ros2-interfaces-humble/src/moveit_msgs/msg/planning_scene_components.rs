use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlanningSceneComponents {
    pub components: u32,
}

impl PlanningSceneComponents {
    pub const SCENE_SETTINGS: u32 = 1;
    pub const ROBOT_STATE: u32 = 2;
    pub const ROBOT_STATE_ATTACHED_OBJECTS: u32 = 4;
    pub const WORLD_OBJECT_NAMES: u32 = 8;
    pub const WORLD_OBJECT_GEOMETRY: u32 = 16;
    pub const OCTOMAP: u32 = 32;
    pub const TRANSFORMS: u32 = 64;
    pub const ALLOWED_COLLISION_MATRIX: u32 = 128;
    pub const LINK_PADDING_AND_SCALING: u32 = 256;
    pub const OBJECT_COLORS: u32 = 512;
}

impl Default for PlanningSceneComponents {
    fn default() -> Self {
        PlanningSceneComponents {
            components: 0,
        }
    }
}

impl ros2_client::Message for PlanningSceneComponents {}
