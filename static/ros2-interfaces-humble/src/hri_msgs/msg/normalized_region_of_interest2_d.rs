use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NormalizedRegionOfInterest2D {
    pub header: crate::std_msgs::msg::Header,
    pub xmin: f32,
    pub ymin: f32,
    pub xmax: f32,
    pub ymax: f32,
    pub c: f32,
}

impl Default for NormalizedRegionOfInterest2D {
    fn default() -> Self {
        NormalizedRegionOfInterest2D {
            header: crate::std_msgs::msg::Header::default(),
            xmin: 0.0,
            ymin: 0.0,
            xmax: 0.0,
            ymax: 0.0,
            c: 0.0,
        }
    }
}

impl ros2_client::Message for NormalizedRegionOfInterest2D {}
