use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParticleCloud {
    pub header: crate::std_msgs::msg::Header,
    pub particles: Vec<crate::nav2_msgs::msg::Particle>,
}

impl Default for ParticleCloud {
    fn default() -> Self {
        ParticleCloud {
            header: crate::std_msgs::msg::Header::default(),
            particles: Vec::new(),
        }
    }
}

impl ros2_client::Message for ParticleCloud {}
