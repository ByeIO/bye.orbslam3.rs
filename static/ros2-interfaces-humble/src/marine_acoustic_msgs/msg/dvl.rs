use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Dvl {
    pub header: crate::std_msgs::msg::Header,
    pub velocity_mode: u8,
    pub dvl_type: u8,
    pub velocity: crate::geometry_msgs::msg::Vector3,
    pub velocity_covar: [f64; 9],
    pub altitude: f64,
    pub course_gnd: f64,
    pub speed_gnd: f64,
    pub num_good_beams: u8,
    pub sound_speed: f32,
    pub beam_ranges_valid: bool,
    pub beam_velocities_valid: bool,
    pub beam_unit_vec: [crate::geometry_msgs::msg::Vector3; 4],
    pub range: [f64; 4],
    pub range_covar: [f32; 4],
    pub beam_quality: [f32; 4],
    pub beam_velocity: [f32; 4],
    pub beam_velocity_covar: [f32; 4],
}

impl Dvl {
    pub const DVL_MODE_BOTTOM: u8 = 1;
    pub const DVL_MODE_WATER: u8 = 2;
    pub const DVL_TYPE_PISTON: u8 = 0;
    pub const DVL_TYPE_PHASED_ARRAY: u8 = 1;
}

impl Default for Dvl {
    fn default() -> Self {
        Dvl {
            header: crate::std_msgs::msg::Header::default(),
            velocity_mode: 0,
            dvl_type: 0,
            velocity: crate::geometry_msgs::msg::Vector3::default(),
            velocity_covar: [0.0; 9],
            altitude: 0.0,
            course_gnd: 0.0,
            speed_gnd: 0.0,
            num_good_beams: 0,
            sound_speed: 0.0,
            beam_ranges_valid: false,
            beam_velocities_valid: false,
            beam_unit_vec: core::array::from_fn(|_| crate::geometry_msgs::msg::Vector3::default()),
            range: [0.0; 4],
            range_covar: [0.0; 4],
            beam_quality: [0.0; 4],
            beam_velocity: [0.0; 4],
            beam_velocity_covar: [0.0; 4],
        }
    }
}

impl ros2_client::Message for Dvl {}
