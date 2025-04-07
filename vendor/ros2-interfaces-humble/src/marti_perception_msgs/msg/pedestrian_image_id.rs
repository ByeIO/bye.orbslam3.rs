use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PedestrianImageID {
    pub header: crate::std_msgs::msg::Header,
    pub pixel_x: i32,
    pub pixel_y: i32,
    pub image_width: i32,
    pub image_height: i32,
}

impl Default for PedestrianImageID {
    fn default() -> Self {
        PedestrianImageID {
            header: crate::std_msgs::msg::Header::default(),
            pixel_x: 0,
            pixel_y: 0,
            image_width: 0,
            image_height: 0,
        }
    }
}

impl ros2_client::Message for PedestrianImageID {}
