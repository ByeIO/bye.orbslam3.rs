use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SbgImuStatus {
    pub imu_com: bool,
    pub imu_status: bool,
    pub imu_accel_x: bool,
    pub imu_accel_y: bool,
    pub imu_accel_z: bool,
    pub imu_gyro_x: bool,
    pub imu_gyro_y: bool,
    pub imu_gyro_z: bool,
    pub imu_accels_in_range: bool,
    pub imu_gyros_in_range: bool,
}

impl Default for SbgImuStatus {
    fn default() -> Self {
        SbgImuStatus {
            imu_com: false,
            imu_status: false,
            imu_accel_x: false,
            imu_accel_y: false,
            imu_accel_z: false,
            imu_gyro_x: false,
            imu_gyro_y: false,
            imu_gyro_z: false,
            imu_accels_in_range: false,
            imu_gyros_in_range: false,
        }
    }
}

impl ros2_client::Message for SbgImuStatus {}
