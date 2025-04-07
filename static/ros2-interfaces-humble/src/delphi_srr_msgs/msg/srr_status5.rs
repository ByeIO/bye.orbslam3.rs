use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SrrStatus5 {
    pub header: crate::std_msgs::msg::Header,
    pub disable_auto_align: bool,
    pub can_tx_yaw_rate_ref_qf: u8,
    pub can_tx_yaw_rate_raw_qf: u8,
    pub can_tx_yaw_rate_reference: f32,
    pub can_tx_yaw_rate_raw: f32,
    pub can_tx_system_status: u8,
    pub can_tx_outside_temperature: i16,
    pub can_blockage_mnr_blocked: bool,
    pub can_blockage_bb_blocked: bool,
    pub can_blockage_radar_blocked: bool,
    pub can_td_blocked: bool,
    pub radar_tx_power_error: bool,
    pub radar_lo_power_error: bool,
    pub radar_data_sync_error: bool,
    pub linearizer_spi_transfer_error: bool,
    pub saturated_tuning_freq_error: bool,
    pub rtn_spi_transfer_error: bool,
    pub rrn_spi_transfer_error: bool,
    pub video_port_capture_error: bool,
    pub vertical_misalignment_error: bool,
    pub tx_temperature_fault: bool,
    pub transmitter_id_error: bool,
    pub dsp_unit_cal_checksum_error: bool,
    pub dsp_unit_cal_block_chcksm_error: bool,
    pub dsp_tuning_sensitivity_error: bool,
    pub dsp_loop_overrun_error: bool,
    pub adc_spi_transfer_error: bool,
}

impl SrrStatus5 {
    pub const CAN_TX_YAW_RATE_REF_QF_UNDEFINED: u8 = 0;
    pub const CAN_TX_YAW_RATE_REF_QF_TEMP_UNDEFINED: u8 = 1;
    pub const CAN_TX_YAW_RATE_REF_QF_NOT_ACCURATE: u8 = 2;
    pub const CAN_TX_YAW_RATE_REF_QF_ACCURATE: u8 = 3;
    pub const CAN_TX_YAW_RATE_RAW_QF_UNDEFINED: u8 = 0;
    pub const CAN_TX_YAW_RATE_RAW_QF_TEMP_UNDEFINED: u8 = 1;
    pub const CAN_TX_YAW_RATE_RAW_QF_NOT_ACCURATE: u8 = 2;
    pub const CAN_TX_YAW_RATE_RAW_QF_ACCURATE: u8 = 3;
    pub const CAN_TX_SYSTEM_STATUS_CONFIGURATION: u8 = 0;
    pub const CAN_TX_SYSTEM_STATUS_STARTUP: u8 = 1;
    pub const CAN_TX_SYSTEM_STATUS_RUNNING: u8 = 2;
    pub const CAN_TX_SYSTEM_STATUS_BLOCKED: u8 = 3;
    pub const CAN_TX_SYSTEM_STATUS_FAULTY: u8 = 4;
    pub const CAN_TX_SYSTEM_STATUS_SHUTDOWN: u8 = 5;
    pub const CAN_TX_SYSTEM_STATUS_HOT: u8 = 6;
}

impl Default for SrrStatus5 {
    fn default() -> Self {
        SrrStatus5 {
            header: crate::std_msgs::msg::Header::default(),
            disable_auto_align: false,
            can_tx_yaw_rate_ref_qf: 0,
            can_tx_yaw_rate_raw_qf: 0,
            can_tx_yaw_rate_reference: 0.0,
            can_tx_yaw_rate_raw: 0.0,
            can_tx_system_status: 0,
            can_tx_outside_temperature: 0,
            can_blockage_mnr_blocked: false,
            can_blockage_bb_blocked: false,
            can_blockage_radar_blocked: false,
            can_td_blocked: false,
            radar_tx_power_error: false,
            radar_lo_power_error: false,
            radar_data_sync_error: false,
            linearizer_spi_transfer_error: false,
            saturated_tuning_freq_error: false,
            rtn_spi_transfer_error: false,
            rrn_spi_transfer_error: false,
            video_port_capture_error: false,
            vertical_misalignment_error: false,
            tx_temperature_fault: false,
            transmitter_id_error: false,
            dsp_unit_cal_checksum_error: false,
            dsp_unit_cal_block_chcksm_error: false,
            dsp_tuning_sensitivity_error: false,
            dsp_loop_overrun_error: false,
            adc_spi_transfer_error: false,
        }
    }
}

impl ros2_client::Message for SrrStatus5 {}
