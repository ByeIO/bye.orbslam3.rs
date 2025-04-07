use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DriverAssist {
    pub header: crate::std_msgs::msg::Header,
    pub decel: f32,
    pub decel_src: crate::ds_dbw_msgs::msg::DecelSrc,
    pub fcw_active: bool,
    pub fcw_enabled: bool,
    pub aeb_active: bool,
    pub aeb_precharge: bool,
    pub aeb_enabled: bool,
    pub acc_braking: bool,
    pub acc_enabled: bool,
    pub blis_l_alert: bool,
    pub blis_l_enabled: bool,
    pub blis_r_alert: bool,
    pub blis_r_enabled: bool,
    pub cta_l_alert: bool,
    pub cta_l_enabled: bool,
    pub cta_r_alert: bool,
    pub cta_r_enabled: bool,
}

impl Default for DriverAssist {
    fn default() -> Self {
        DriverAssist {
            header: crate::std_msgs::msg::Header::default(),
            decel: 0.0,
            decel_src: crate::ds_dbw_msgs::msg::DecelSrc::default(),
            fcw_active: false,
            fcw_enabled: false,
            aeb_active: false,
            aeb_precharge: false,
            aeb_enabled: false,
            acc_braking: false,
            acc_enabled: false,
            blis_l_alert: false,
            blis_l_enabled: false,
            blis_r_alert: false,
            blis_r_enabled: false,
            cta_l_alert: false,
            cta_l_enabled: false,
            cta_r_alert: false,
            cta_r_enabled: false,
        }
    }
}

impl ros2_client::Message for DriverAssist {}
