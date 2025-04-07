use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RobotStateRTMsg {
    pub time: f64,
    pub q_target: Vec<f64>,
    pub qd_target: Vec<f64>,
    pub qdd_target: Vec<f64>,
    pub i_target: Vec<f64>,
    pub m_target: Vec<f64>,
    pub q_actual: Vec<f64>,
    pub qd_actual: Vec<f64>,
    pub i_actual: Vec<f64>,
    pub tool_acc_values: Vec<f64>,
    pub tcp_force: Vec<f64>,
    pub tool_vector: Vec<f64>,
    pub tcp_speed: Vec<f64>,
    pub digital_input_bits: f64,
    pub motor_temperatures: Vec<f64>,
    pub controller_timer: f64,
    pub test_value: f64,
    pub robot_mode: f64,
    pub joint_modes: Vec<f64>,
}

impl Default for RobotStateRTMsg {
    fn default() -> Self {
        RobotStateRTMsg {
            time: 0.0,
            q_target: Vec::new(),
            qd_target: Vec::new(),
            qdd_target: Vec::new(),
            i_target: Vec::new(),
            m_target: Vec::new(),
            q_actual: Vec::new(),
            qd_actual: Vec::new(),
            i_actual: Vec::new(),
            tool_acc_values: Vec::new(),
            tcp_force: Vec::new(),
            tool_vector: Vec::new(),
            tcp_speed: Vec::new(),
            digital_input_bits: 0.0,
            motor_temperatures: Vec::new(),
            controller_timer: 0.0,
            test_value: 0.0,
            robot_mode: 0.0,
            joint_modes: Vec::new(),
        }
    }
}

impl ros2_client::Message for RobotStateRTMsg {}
