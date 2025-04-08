use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LocationAttributes {
    pub header: crate::std_msgs::msg::Header,
    pub location_attributes_header: crate::off_highway_premium_radar_sample_msgs::msg::LocationAttributesHeader,
    pub location_attributes_packet: crate::off_highway_premium_radar_sample_msgs::msg::LocationAttributesPacket,
    pub mounting_pose: crate::geometry_msgs::msg::Pose,
}

impl Default for LocationAttributes {
    fn default() -> Self {
        LocationAttributes {
            header: crate::std_msgs::msg::Header::default(),
            location_attributes_header: crate::off_highway_premium_radar_sample_msgs::msg::LocationAttributesHeader::default(),
            location_attributes_packet: crate::off_highway_premium_radar_sample_msgs::msg::LocationAttributesPacket::default(),
            mounting_pose: crate::geometry_msgs::msg::Pose::default(),
        }
    }
}

impl ros2_client::Message for LocationAttributes {}
