use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UBXEsfStatus {
    pub header: crate::std_msgs::msg::Header,
    pub itow: u32,
    pub version: u8,
    pub wt_init_status: u8,
    pub mnt_alg_status: u8,
    pub ins_init_status: u8,
    pub imu_init_status: u8,
    pub fusion_mode: u8,
    pub num_sens: u8,
    pub sensor_statuses: Vec<crate::ublox_ubx_msgs::msg::ESFSensorStatus>,
}

impl UBXEsfStatus {
    pub const WT_INIT_STATUS_OFF: u8 = 0;
    pub const WT_INIT_STATUS_INITIALIZING: u8 = 1;
    pub const WT_INIT_STATUS_INITIALIZED: u8 = 2;
    pub const MBT_ALG_STATUS_OFF: u8 = 0;
    pub const MBT_ALG_STATUS_INITIALIZING: u8 = 1;
    pub const MBT_ALG_STATUS_INITIALIZED0: u8 = 2;
    pub const MBT_ALG_STATUS_INITIALIZED1: u8 = 3;
    pub const INS_INIT_STATUS_OFF: u8 = 0;
    pub const INS_INIT_STATUS_INITIALIZING: u8 = 1;
    pub const INS_INIT_STATUS_INITIALIZED: u8 = 2;
    pub const IMU_INIT_STATUS_OFF: u8 = 0;
    pub const IMU_INIT_STATUS_INITIALIZING: u8 = 1;
    pub const IMU_INIT_STATUS_INITIALIZED: u8 = 2;
    pub const FUSION_MODE_INITIALIZATION: u8 = 0;
    pub const FUSION_MODE_WORKING: u8 = 1;
    pub const FUSION_MODE_SUSPENDED: u8 = 2;
    pub const FUSION_MODE_DISABLED: u8 = 3;
}

impl Default for UBXEsfStatus {
    fn default() -> Self {
        UBXEsfStatus {
            header: crate::std_msgs::msg::Header::default(),
            itow: 0,
            version: 0,
            wt_init_status: 0,
            mnt_alg_status: 0,
            ins_init_status: 0,
            imu_init_status: 0,
            fusion_mode: 0,
            num_sens: 0,
            sensor_statuses: Vec::new(),
        }
    }
}

impl ros2_client::Message for UBXEsfStatus {}
