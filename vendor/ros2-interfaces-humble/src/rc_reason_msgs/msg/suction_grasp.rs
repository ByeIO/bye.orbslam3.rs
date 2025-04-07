use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SuctionGrasp {
    pub uuid: ::std::string::String,
    pub item_uuid: ::std::string::String,
    pub pose: crate::geometry_msgs::msg::PoseStamped,
    pub quality: f64,
    pub max_suction_surface_length: f64,
    pub max_suction_surface_width: f64,
}

impl Default for SuctionGrasp {
    fn default() -> Self {
        SuctionGrasp {
            uuid: ::std::string::String::new(),
            item_uuid: ::std::string::String::new(),
            pose: crate::geometry_msgs::msg::PoseStamped::default(),
            quality: 0.0,
            max_suction_surface_length: 0.0,
            max_suction_surface_width: 0.0,
        }
    }
}

impl ros2_client::Message for SuctionGrasp {}
