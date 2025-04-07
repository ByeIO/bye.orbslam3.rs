use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AudioFeatures {
    pub zcr: f32,
    pub rms: f32,
    pub pitch: f32,
    pub hnr: f32,
    pub mfcc: Vec<f32>,
}

impl Default for AudioFeatures {
    fn default() -> Self {
        AudioFeatures {
            zcr: 0.0,
            rms: 0.0,
            pitch: 0.0,
            hnr: 0.0,
            mfcc: Vec::new(),
        }
    }
}

impl ros2_client::Message for AudioFeatures {}
