use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EsrVehicle3 {
    pub header: crate::std_msgs::msg::Header,
    pub long_accel_validity: bool,
    pub lat_accel_validity: bool,
    pub lat_accel: f32,
    pub long_accel: f32,
    pub radar_fov_lr: u8,
    pub radar_fov_mr: u8,
    pub auto_align_disable: bool,
    pub radar_height: u8,
    pub serv_align_type: bool,
    pub serv_align_enable: bool,
    pub aalign_avg_ctr_total: u16,
    pub auto_align_converged: bool,
    pub wheel_slip: u8,
    pub serv_align_updates_need: u8,
    pub angle_mounting_offset: i8,
}

impl Default for EsrVehicle3 {
    fn default() -> Self {
        EsrVehicle3 {
            header: crate::std_msgs::msg::Header::default(),
            long_accel_validity: false,
            lat_accel_validity: false,
            lat_accel: 0.0,
            long_accel: 0.0,
            radar_fov_lr: 0,
            radar_fov_mr: 0,
            auto_align_disable: false,
            radar_height: 0,
            serv_align_type: false,
            serv_align_enable: false,
            aalign_avg_ctr_total: 0,
            auto_align_converged: false,
            wheel_slip: 0,
            serv_align_updates_need: 0,
            angle_mounting_offset: 0,
        }
    }
}

impl ros2_client::Message for EsrVehicle3 {}
