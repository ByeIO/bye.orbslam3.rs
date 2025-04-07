use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AudioNoteVector {
    pub header: crate::std_msgs::msg::Header,
    pub notes: Vec<crate::irobot_create_msgs::msg::AudioNote>,
    pub append: bool,
}

impl Default for AudioNoteVector {
    fn default() -> Self {
        AudioNoteVector {
            header: crate::std_msgs::msg::Header::default(),
            notes: Vec::new(),
            append: false,
        }
    }
}

impl ros2_client::Message for AudioNoteVector {}
