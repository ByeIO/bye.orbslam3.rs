#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(non_fmt_panics)]
#![allow(unused_mut)]
#![allow(unused_assignments)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(rustdoc::missing_crate_level_docs)]
#![allow(unsafe_code)]
#![allow(clippy::undocumented_unsafe_blocks)]
#![allow(unused_must_use)]
#![allow(non_snake_case)]
#![allow(unused_doc_comments)]
use std::f64::consts::E;
use std::f64::consts::PI;

/* start 数值转换 */
// pub const PI: f64 = 3.14159265358979323846;
pub const PI_2: f64 = 1.570796326794896619;

/// 将度转换为弧度
pub fn deg_to_rad(deg: f64) -> f64 {
    deg * 0.01745329251994329575
}

/// 将弧度转换为度
pub fn rad_to_deg(rad: f64) -> f64 {
    rad * 57.29577951308232087721
}

/// 从概率计算对数几率
pub fn logodds(probability: f64) -> f64 {
    (probability / (1.0_f64 - probability)).ln() as f64
}

/// 从对数几率计算概率
pub fn probability(logodds: f64) -> f64 {
    1.0 - (1.0 / (1.0 + E.powf(logodds)))
}
/* end 数值转换 */

#[cfg(test)]
mod tests1 {
    use super::*;

    #[test]
    fn test_logodds() {
        let prob = 0.7_f64;
        let expected = (0.7_f64 / 0.3_f64).ln() as f64;
        println!("{}", expected);
        let result = logodds(prob);
        println!("{}", result);
        // f64浮点数精度会导致存在误差
        assert!(result - expected < 1e-6);
    }

    #[test]
    fn test_probability() {
        let lodds = 1.0;
        let expected = 1.0 - (1.0 / (1.0 + std::f64::consts::E.powf(1.0)));
        assert!(probability(lodds) - expected < 1e-6);
    }
}
