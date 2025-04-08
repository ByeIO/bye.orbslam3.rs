use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScanPoint {
    pub angle: f32,
    pub distance: u16,
    pub reflectivity: u8,
    pub valid: bool,
    pub infinite: bool,
    pub glare: bool,
    pub reflector: bool,
    pub contamination: bool,
    pub contamination_warning: bool,
}

impl Default for ScanPoint {
    fn default() -> Self {
        ScanPoint {
            angle: 0.0,
            distance: 0,
            reflectivity: 0,
            valid: false,
            infinite: false,
            glare: false,
            reflector: false,
            contamination: false,
            contamination_warning: false,
        }
    }
}

impl ros2_client::Message for ScanPoint {}
