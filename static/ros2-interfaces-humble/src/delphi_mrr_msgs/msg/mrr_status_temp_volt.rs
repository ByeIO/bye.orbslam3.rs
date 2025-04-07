use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MrrStatusTempVolt {
    pub header: crate::std_msgs::msg::Header,
    pub can_batt_volts: f32,
    pub can_1_25_v: f32,
    pub can_5_v: f32,
    pub can_3_3_v_raw: f32,
    pub can_3_3_v_dac: f32,
    pub can_mmic_temp1: i8,
    pub can_processor_thermistor: i8,
    pub can_processor_temp1: i8,
}

impl Default for MrrStatusTempVolt {
    fn default() -> Self {
        MrrStatusTempVolt {
            header: crate::std_msgs::msg::Header::default(),
            can_batt_volts: 0.0,
            can_1_25_v: 0.0,
            can_5_v: 0.0,
            can_3_3_v_raw: 0.0,
            can_3_3_v_dac: 0.0,
            can_mmic_temp1: 0,
            can_processor_thermistor: 0,
            can_processor_temp1: 0,
        }
    }
}

impl ros2_client::Message for MrrStatusTempVolt {}
