use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SensorData {
    pub header: crate::std_msgs::msg::Header,
    pub left: crate::sensor_msgs::msg::Image,
    pub right: crate::sensor_msgs::msg::Image,
    pub left_compressed: Vec<u8>,
    pub right_compressed: Vec<u8>,
    pub left_camera_info: Vec<crate::sensor_msgs::msg::CameraInfo>,
    pub right_camera_info: Vec<crate::sensor_msgs::msg::CameraInfo>,
    pub local_transform: Vec<crate::geometry_msgs::msg::Transform>,
    pub laser_scan: crate::sensor_msgs::msg::PointCloud2,
    pub laser_scan_compressed: Vec<u8>,
    pub laser_scan_max_pts: i32,
    pub laser_scan_max_range: f32,
    pub laser_scan_format: i32,
    pub laser_scan_local_transform: crate::geometry_msgs::msg::Transform,
    pub user_data: Vec<u8>,
    pub grid_ground: Vec<u8>,
    pub grid_obstacles: Vec<u8>,
    pub grid_empty_cells: Vec<u8>,
    pub grid_cell_size: f32,
    pub grid_view_point: crate::rtabmap_msgs::msg::Point3f,
    pub key_points: Vec<crate::rtabmap_msgs::msg::KeyPoint>,
    pub points: Vec<crate::rtabmap_msgs::msg::Point3f>,
    pub descriptors: Vec<u8>,
    pub global_descriptors: Vec<crate::rtabmap_msgs::msg::GlobalDescriptor>,
    pub env_sensors: Vec<crate::rtabmap_msgs::msg::EnvSensor>,
    pub imu: crate::sensor_msgs::msg::Imu,
    pub imu_local_transform: crate::geometry_msgs::msg::Transform,
    pub landmarks: Vec<crate::rtabmap_msgs::msg::LandmarkDetection>,
    pub ground_truth_pose: crate::geometry_msgs::msg::Pose,
    pub gps: crate::rtabmap_msgs::msg::GPS,
}

impl Default for SensorData {
    fn default() -> Self {
        SensorData {
            header: crate::std_msgs::msg::Header::default(),
            left: crate::sensor_msgs::msg::Image::default(),
            right: crate::sensor_msgs::msg::Image::default(),
            left_compressed: Vec::new(),
            right_compressed: Vec::new(),
            left_camera_info: Vec::new(),
            right_camera_info: Vec::new(),
            local_transform: Vec::new(),
            laser_scan: crate::sensor_msgs::msg::PointCloud2::default(),
            laser_scan_compressed: Vec::new(),
            laser_scan_max_pts: 0,
            laser_scan_max_range: 0.0,
            laser_scan_format: 0,
            laser_scan_local_transform: crate::geometry_msgs::msg::Transform::default(),
            user_data: Vec::new(),
            grid_ground: Vec::new(),
            grid_obstacles: Vec::new(),
            grid_empty_cells: Vec::new(),
            grid_cell_size: 0.0,
            grid_view_point: crate::rtabmap_msgs::msg::Point3f::default(),
            key_points: Vec::new(),
            points: Vec::new(),
            descriptors: Vec::new(),
            global_descriptors: Vec::new(),
            env_sensors: Vec::new(),
            imu: crate::sensor_msgs::msg::Imu::default(),
            imu_local_transform: crate::geometry_msgs::msg::Transform::default(),
            landmarks: Vec::new(),
            ground_truth_pose: crate::geometry_msgs::msg::Pose::default(),
            gps: crate::rtabmap_msgs::msg::GPS::default(),
        }
    }
}

impl ros2_client::Message for SensorData {}
