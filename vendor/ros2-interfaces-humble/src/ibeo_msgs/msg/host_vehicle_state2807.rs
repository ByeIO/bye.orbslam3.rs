use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HostVehicleState2807 {
    pub header: crate::std_msgs::msg::Header,
    pub ibeo_header: crate::ibeo_msgs::msg::IbeoDataHeader,
    pub timestamp: crate::builtin_interfaces::msg::Time,
    pub distance_x: i32,
    pub distance_y: i32,
    pub course_angle: f32,
    pub longitudinal_velocity: f32,
    pub yaw_rate: f32,
    pub steering_wheel_angle: f32,
    pub cross_acceleration: f32,
    pub front_wheel_angle: f32,
    pub vehicle_width: f32,
    pub vehicle_front_to_front_axle: f32,
    pub rear_axle_to_front_axle: f32,
    pub rear_axle_to_vehicle_rear: f32,
    pub steer_ratio_poly_0: f32,
    pub steer_ratio_poly_1: f32,
    pub steer_ratio_poly_2: f32,
    pub steer_ratio_poly_3: f32,
    pub longitudinal_acceleration: f32,
}

impl Default for HostVehicleState2807 {
    fn default() -> Self {
        HostVehicleState2807 {
            header: crate::std_msgs::msg::Header::default(),
            ibeo_header: crate::ibeo_msgs::msg::IbeoDataHeader::default(),
            timestamp: crate::builtin_interfaces::msg::Time::default(),
            distance_x: 0,
            distance_y: 0,
            course_angle: 0.0,
            longitudinal_velocity: 0.0,
            yaw_rate: 0.0,
            steering_wheel_angle: 0.0,
            cross_acceleration: 0.0,
            front_wheel_angle: 0.0,
            vehicle_width: 0.0,
            vehicle_front_to_front_axle: 0.0,
            rear_axle_to_front_axle: 0.0,
            rear_axle_to_vehicle_rear: 0.0,
            steer_ratio_poly_0: 0.0,
            steer_ratio_poly_1: 0.0,
            steer_ratio_poly_2: 0.0,
            steer_ratio_poly_3: 0.0,
            longitudinal_acceleration: 0.0,
        }
    }
}

impl ros2_client::Message for HostVehicleState2807 {}
