use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VelSensorSetup {
    pub header: crate::std_msgs::msg::Header,
    pub block_header: crate::septentrio_gnss_driver::msg::BlockHeader,
    pub port: u8,
    pub lever_arm_x: f32,
    pub lever_arm_y: f32,
    pub lever_arm_z: f32,
}

impl Default for VelSensorSetup {
    fn default() -> Self {
        VelSensorSetup {
            header: crate::std_msgs::msg::Header::default(),
            block_header: crate::septentrio_gnss_driver::msg::BlockHeader::default(),
            port: 0,
            lever_arm_x: 0.0,
            lever_arm_y: 0.0,
            lever_arm_z: 0.0,
        }
    }
}

impl ros2_client::Message for VelSensorSetup {}
