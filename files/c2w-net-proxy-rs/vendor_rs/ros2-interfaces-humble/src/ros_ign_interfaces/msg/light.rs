use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Light {
    pub header: crate::std_msgs::msg::Header,
    pub name: ::std::string::String,
    #[serde(rename = "type")]    pub type_: u8,
    pub pose: crate::geometry_msgs::msg::Pose,
    pub diffuse: crate::std_msgs::msg::ColorRGBA,
    pub specular: crate::std_msgs::msg::ColorRGBA,
    pub attenuation_constant: f32,
    pub attenuation_linear: f32,
    pub attenuation_quadratic: f32,
    pub direction: crate::geometry_msgs::msg::Vector3,
    pub range: f32,
    pub cast_shadows: bool,
    pub spot_inner_angle: f32,
    pub spot_outer_angle: f32,
    pub spot_falloff: f32,
    pub id: u32,
    pub parent_id: u32,
    pub intensity: f32,
}

impl Light {
    pub const POINT: u8 = 0;
    pub const SPOT: u8 = 1;
    pub const DIRECTIONAL: u8 = 2;
}

impl Default for Light {
    fn default() -> Self {
        Light {
            header: crate::std_msgs::msg::Header::default(),
            name: ::std::string::String::new(),
            type_: 0,
            pose: crate::geometry_msgs::msg::Pose::default(),
            diffuse: crate::std_msgs::msg::ColorRGBA::default(),
            specular: crate::std_msgs::msg::ColorRGBA::default(),
            attenuation_constant: 0.0,
            attenuation_linear: 0.0,
            attenuation_quadratic: 0.0,
            direction: crate::geometry_msgs::msg::Vector3::default(),
            range: 0.0,
            cast_shadows: false,
            spot_inner_angle: 0.0,
            spot_outer_angle: 0.0,
            spot_falloff: 0.0,
            id: 0,
            parent_id: 0,
            intensity: 0.0,
        }
    }
}

impl ros2_client::Message for Light {}
