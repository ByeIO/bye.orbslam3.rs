use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LeadVehicle {
    pub header: crate::std_msgs::msg::Header,
    pub headway_distance: f32,
    pub speed: f32,
    pub heading: f32,
    pub x_pos: f32,
    pub y_pos: f32,
    pub classification: i8,
    #[serde(rename = "type")]    pub type_: i8,
}

impl Default for LeadVehicle {
    fn default() -> Self {
        LeadVehicle {
            header: crate::std_msgs::msg::Header::default(),
            headway_distance: 0.0,
            speed: 0.0,
            heading: 0.0,
            x_pos: 0.0,
            y_pos: 0.0,
            classification: 0,
            type_: 0,
        }
    }
}

impl ros2_client::Message for LeadVehicle {}
