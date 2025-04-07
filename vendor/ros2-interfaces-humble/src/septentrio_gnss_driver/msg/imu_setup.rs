use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IMUSetup {
    pub header: crate::std_msgs::msg::Header,
    pub block_header: crate::septentrio_gnss_driver::msg::BlockHeader,
    pub serial_port: u8,
    pub ant_lever_arm_x: f32,
    pub ant_lever_arm_y: f32,
    pub ant_lever_arm_z: f32,
    pub theta_x: f32,
    pub theta_y: f32,
    pub theta_z: f32,
}

impl Default for IMUSetup {
    fn default() -> Self {
        IMUSetup {
            header: crate::std_msgs::msg::Header::default(),
            block_header: crate::septentrio_gnss_driver::msg::BlockHeader::default(),
            serial_port: 0,
            ant_lever_arm_x: 0.0,
            ant_lever_arm_y: 0.0,
            ant_lever_arm_z: 0.0,
            theta_x: 0.0,
            theta_y: 0.0,
            theta_z: 0.0,
        }
    }
}

impl ros2_client::Message for IMUSetup {}
