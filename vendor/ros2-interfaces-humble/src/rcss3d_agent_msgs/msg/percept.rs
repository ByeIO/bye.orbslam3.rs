use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Percept {
    pub gyro_rates: Vec<crate::rcss3d_agent_msgs::msg::GyroRate>,
    pub hinge_joints: Vec<crate::rcss3d_agent_msgs::msg::HingeJointPos>,
    pub universal_joints: Vec<crate::rcss3d_agent_msgs::msg::UniversalJointPos>,
    pub force_resistances: Vec<crate::rcss3d_agent_msgs::msg::ForceResistance>,
    pub accelerometers: Vec<crate::rcss3d_agent_msgs::msg::Accelerometer>,
    pub vision: Vec<crate::rcss3d_agent_msgs::msg::Vision>,
    pub game_state: crate::rcss3d_agent_msgs::msg::GameState,
    pub agent_state: Vec<crate::rcss3d_agent_msgs::msg::AgentState>,
    pub hears: Vec<crate::rcss3d_agent_msgs::msg::Hear>,
}

impl Default for Percept {
    fn default() -> Self {
        Percept {
            gyro_rates: Vec::new(),
            hinge_joints: Vec::new(),
            universal_joints: Vec::new(),
            force_resistances: Vec::new(),
            accelerometers: Vec::new(),
            vision: Vec::new(),
            game_state: crate::rcss3d_agent_msgs::msg::GameState::default(),
            agent_state: Vec::new(),
            hears: Vec::new(),
        }
    }
}

impl ros2_client::Message for Percept {}
