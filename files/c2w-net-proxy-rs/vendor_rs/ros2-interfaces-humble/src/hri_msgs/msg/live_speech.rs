use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LiveSpeech {
    pub header: crate::std_msgs::msg::Header,
    pub incremental: ::std::string::String,
    #[serde(rename = "final")]    pub final_: ::std::string::String,
    pub confidence: f64,
    pub locale: ::std::string::String,
    pub language: ::std::string::String,
}

impl Default for LiveSpeech {
    fn default() -> Self {
        LiveSpeech {
            header: crate::std_msgs::msg::Header::default(),
            incremental: ::std::string::String::new(),
            final_: ::std::string::String::new(),
            confidence: 0.0,
            locale: ::std::string::String::new(),
            language: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for LiveSpeech {}
