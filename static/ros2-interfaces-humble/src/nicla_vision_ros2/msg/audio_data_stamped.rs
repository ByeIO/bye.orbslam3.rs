use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AudioDataStamped {
    pub header: crate::std_msgs::msg::Header,
    pub audio: crate::nicla_vision_ros2::msg::AudioData,
}

impl Default for AudioDataStamped {
    fn default() -> Self {
        AudioDataStamped {
            header: crate::std_msgs::msg::Header::default(),
            audio: crate::nicla_vision_ros2::msg::AudioData::default(),
        }
    }
}

impl ros2_client::Message for AudioDataStamped {}
