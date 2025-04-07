use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Base2DKinematics {
    pub min_vel_x: f64,
    pub max_vel_x: f64,
    pub min_vel_y: f64,
    pub max_vel_y: f64,
    pub max_vel_theta: f64,
    pub acc_lim_x: f64,
    pub decel_lim_x: f64,
    pub acc_lim_y: f64,
    pub decel_lim_y: f64,
    pub acc_lim_theta: f64,
    pub decel_lim_theta: f64,
    pub min_speed_xy: f64,
    pub max_speed_xy: f64,
    pub min_speed_theta: f64,
}

impl Default for Base2DKinematics {
    fn default() -> Self {
        Base2DKinematics {
            min_vel_x: 0.0,
            max_vel_x: 0.0,
            min_vel_y: 0.0,
            max_vel_y: 0.0,
            max_vel_theta: 0.0,
            acc_lim_x: 0.0,
            decel_lim_x: 0.0,
            acc_lim_y: 0.0,
            decel_lim_y: 0.0,
            acc_lim_theta: 0.0,
            decel_lim_theta: 0.0,
            min_speed_xy: 0.0,
            max_speed_xy: 0.0,
            min_speed_theta: 0.0,
        }
    }
}

impl ros2_client::Message for Base2DKinematics {}
