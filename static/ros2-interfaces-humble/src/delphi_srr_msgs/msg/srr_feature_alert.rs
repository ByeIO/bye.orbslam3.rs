use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SrrFeatureAlert {
    pub header: crate::std_msgs::msg::Header,
    pub lcma_blis_ignored_track_id: u8,
    pub lcma_blis_track_id: u8,
    pub lcma_cvw_ttc: f32,
    pub cta_ttc_alert: bool,
    pub cta_selected_track_ttc: f32,
    pub cta_selected_track: u16,
    pub cta_alert: u8,
    pub cta_active: bool,
    pub lcma_cvw_cipv: u8,
    pub lcma_cvw_alert_state: u8,
    pub lcma_blis_alert_state: u8,
    pub lcma_active: bool,
}

impl Default for SrrFeatureAlert {
    fn default() -> Self {
        SrrFeatureAlert {
            header: crate::std_msgs::msg::Header::default(),
            lcma_blis_ignored_track_id: 0,
            lcma_blis_track_id: 0,
            lcma_cvw_ttc: 0.0,
            cta_ttc_alert: false,
            cta_selected_track_ttc: 0.0,
            cta_selected_track: 0,
            cta_alert: 0,
            cta_active: false,
            lcma_cvw_cipv: 0,
            lcma_cvw_alert_state: 0,
            lcma_blis_alert_state: 0,
            lcma_active: false,
        }
    }
}

impl ros2_client::Message for SrrFeatureAlert {}
