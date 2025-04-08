use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Expression {
    pub header: crate::std_msgs::msg::Header,
    pub expression: ::std::string::String,
    pub valence: f32,
    pub arousal: f32,
    pub confidence: f32,
}

impl Expression {
    pub const NEUTRAL: &'static str = "neutral";
    pub const ANGRY: &'static str = "angry";
    pub const SAD: &'static str = "sad";
    pub const HAPPY: &'static str = "happy";
    pub const SURPRISED: &'static str = "surprised";
    pub const DISGUSTED: &'static str = "disgusted";
    pub const SCARED: &'static str = "scared";
    pub const PLEADING: &'static str = "pleading";
    pub const VULNERABLE: &'static str = "vulnerable";
    pub const DESPAIRED: &'static str = "despaired";
    pub const GUILTY: &'static str = "guilty";
    pub const DISAPPOINTED: &'static str = "disappointed";
    pub const EMBARRASSED: &'static str = "embarrassed";
    pub const HORRIFIED: &'static str = "horrified";
    pub const SKEPTICAL: &'static str = "skeptical";
    pub const ANNOYED: &'static str = "annoyed";
    pub const FURIOUS: &'static str = "furious";
    pub const SUSPICIOUS: &'static str = "suspicious";
    pub const REJECTED: &'static str = "rejected";
    pub const BORED: &'static str = "bored";
    pub const TIRED: &'static str = "tired";
    pub const ASLEEP: &'static str = "asleep";
    pub const CONFUSED: &'static str = "confused";
    pub const AMAZED: &'static str = "amazed";
    pub const EXCITED: &'static str = "excited";
}

impl Default for Expression {
    fn default() -> Self {
        Expression {
            header: crate::std_msgs::msg::Header::default(),
            expression: ::std::string::String::new(),
            valence: 0.0,
            arousal: 0.0,
            confidence: 0.0,
        }
    }
}

impl ros2_client::Message for Expression {}
