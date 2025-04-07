# struct,impl及fun签名

## Types.rs
**crate::Types**
```rs
pub type Point3d = Vector3;
pub type Pose6d = Pose6D;
pub type Point3dCollection = Vec<Vector3>;
pub type Point3dList = std::collections::LinkedList<Vector3>;
// OcTreeVolume struct
pub struct OcTreeVolume {
    pub center: Point3d,
    pub side_length: f64,
}
// Vector3 struct and impl
pub struct Vector3 {
    pub inner: na_Vector3<f64>,
}

impl Vector3 {
    pub fn new() -> Self;
    pub fn from_xyz(x: f64, y: f64, z: f64) -> Self;
    pub fn cross(&self, other: &Self) -> Self;
    pub fn dot(&self, other: &Self) -> f64;
    pub fn x(&self) -> f64;
    pub fn y(&self) -> f64;
    pub fn z(&self) -> f64;
    pub fn neg(&self) -> Self;
    pub fn norm(&self) -> f64;
    pub fn norm_sq(&self) -> f64;
    pub fn normalize(&mut self);
    pub fn normalized(&self) -> Self;
    pub fn angle_to(&self, other: &Self) -> f64;
    pub fn distance(&self, other: &Self) -> f64;
    pub fn distance_xy(&self, other: &Self) -> f64;
}

// Quaternion struct and impl
pub struct Quaternion {
    pub inner: na_UnitQuaternion<f64>,
}

impl Quaternion {
    pub fn new() -> Self;
    pub fn from_uxyz(u: f64, x: f64, y: f64, z: f64) -> Self;
    pub fn from_euler(roll: f64, pitch: f64, yaw: f64) -> Self;
    pub fn from_axis_angle(axis: &Vector3, angle: f64) -> Self;
    pub fn to_euler(&self) -> Vector3;
    pub fn normalize(&mut self);
    pub fn normalized(&self) -> Self;
    pub fn inv(&self) -> Self;
    pub fn rotate(&self, v: &Vector3) -> Vector3;
    pub fn u(&self) -> f64;
    pub fn x(&self) -> f64;
    pub fn y(&self) -> f64;
    pub fn z(&self) -> f64;
}

// Pose6D struct and impl
pub struct Pose6D {
    pub translation: Vector3,
    pub rotation: Quaternion,
}

impl Pose6D {
    pub fn new() -> Self;
    pub fn from_trans_rot(trans: Vector3, rot: Quaternion) -> Self;
    pub fn from_xyz_rpy(x: f64, y: f64, z: f64, roll: f64, pitch: f64, yaw: f64) -> Self;
    pub fn trans(&self) -> &Vector3;
    pub fn rot(&self) -> &Quaternion;
    pub fn x(&self) -> f64;
    pub fn y(&self) -> f64;
    pub fn z(&self) -> f64;
    pub fn roll(&self) -> f64;
    pub fn pitch(&self) -> f64;
    pub fn yaw(&self) -> f64;
    pub fn transform(&self, v: &Vector3) -> Vector3;
    pub fn inv(&self) -> Self;
    pub fn inv_ip(&mut self) -> &mut Self;
    pub fn distance(&self, other: &Self) -> f64;
    pub fn trans_length(&self) -> f64;
}
```

## Utils.rs
**crate::Utils**
```rs
pub fn deg_to_rad(deg: f64) -> f64;
pub fn rad_to_deg(rad: f64) -> f64;
pub fn logodds(probability: f64) -> f64;
pub fn probability(logodds: f64) -> f64;
```

## PointCloud.rs
**crate::PointCloud**
```rs
pub struct Pointcloud {
    pub points: Point3dCollection,
    pub current_inv_transform: Pose6d,
}

impl Pointcloud {
    pub fn new() -> Self;
    pub fn clear(&mut self);
    pub fn size(&self) -> usize;
    pub fn reserve(&mut self, size: usize);
    pub fn push_back(&mut self, x: f64, y: f64, z: f64);
    pub fn push_back_ref(&mut self, p: &Point3d);
    pub fn push_back_cloud(&mut self, other: &Pointcloud);
    pub fn transform(&mut self, transform: Pose6d);
    pub fn transform_absolute(&mut self, transform: Pose6d);
    pub fn rotate(&mut self, roll: f64, pitch: f64, yaw: f64);
    pub fn calc_bbx(&self) -> (Point3d, Point3d);
    pub fn crop(&mut self, lower_bound: Point3d, upper_bound: Point3d);
    pub fn min_dist(&mut self, thres: f64);
    pub fn sub_sample_random(&self, num_samples: usize) -> Pointcloud;
    pub fn write_vrml(&self, filename: &Path) -> io::Result<()>;
}
```

## ScanGraph.rs
**crate::ScanGraph**
```rs
pub struct ScanNode {
    pub scan: Option<Pointcloud>,
    pub pose: Pose6d,
    pub id: u32,
}
impl ScanNode {
    pub fn new(scan: Option<Pointcloud>, pose: Pose6d, id: u32) -> Self;
    pub fn default() -> Self;
    pub fn write_binary(&self, s: &mut impl Write) -> io::Result<()>;
    pub fn read_binary(&mut self, s: &mut impl Read) -> io::Result<()>;
    pub fn write_pose_ascii(&self, s: &mut impl Write) -> io::Result<()>;
    pub fn read_pose_ascii(&mut self, s: &mut impl Read) -> io::Result<()>;
}
impl PartialEq for ScanNode {
    fn eq(&self, other: &Self) -> bool;
}
impl Clone for ScanNode {
    fn clone(&self) -> Self;
}
pub struct ScanEdge {
    pub first: Rc<RefCell<ScanNode>>,
    pub second: Rc<RefCell<ScanNode>>,
    pub constraint: Pose6d,
    pub weight: f64,
}
impl ScanEdge {
    pub fn new(first: Rc<RefCell<ScanNode>>, second: Rc<RefCell<ScanNode>>, constraint: Pose6d) -> Self;
    pub fn default() -> Self;
    pub fn write_binary(&self, s: &mut impl Write) -> io::Result<()>;
    pub fn read_binary(&mut self, s: &mut impl Read, graph: &ScanGraph) -> io::Result<()>;
}
impl PartialEq for ScanEdge {
    fn eq(&self, other: &Self) -> bool;
}
impl Clone for ScanEdge {
    fn clone(&self) -> Self;
}
pub struct ScanGraph {
    pub nodes: Vec<Rc<RefCell<ScanNode>>>,
    pub edges: Vec<Rc<RefCell<ScanEdge>>>,
}
impl ScanGraph {
    pub fn new() -> Self;
    pub fn clear(&mut self);
    pub fn add_node(&mut self, scan: Option<Pointcloud>, pose: Pose6d) -> Rc<RefCell<ScanNode>>;
    pub fn add_edge(&mut self, first: Rc<RefCell<ScanNode>>, second: Rc<RefCell<ScanNode>>, constraint: Pose6d) -> Rc<RefCell<ScanEdge>>;
    pub fn connect_previous(&mut self);
    pub fn export_dot(&self, filename: &str) -> io::Result<()>;
    pub fn get_node_by_id(&self, id: u32) -> Option<Rc<RefCell<ScanNode>>>;
    pub fn edge_exists(&self, first_id: u32, second_id: u32) -> bool;
    pub fn get_neighbor_ids(&self, id: u32) -> Vec<u32>;
    pub fn get_out_edges(&self, node: Rc<RefCell<ScanNode>>) -> Vec<Rc<RefCell<ScanEdge>>>;
    pub fn get_in_edges(&self, node: Rc<RefCell<ScanNode>>) -> Vec<Rc<RefCell<ScanEdge>>>;
    pub fn transform_scans(&self);
    pub fn crop(&self, lower_bound: Point3d, upper_bound: Point3d);
    pub fn get_num_points(&self, max_id: u32) -> usize;
    pub fn write_binary(&self, filename: &str) -> io::Result<()>;
    fn write_binary_internal(&self, writer: &mut impl Write) -> io::Result<()>;
    pub fn read_binary(&mut self, filename: &str) -> io::Result<()>;
    fn read_binary_internal(&mut self, reader: &mut impl Read) -> io::Result<()>;
    pub fn write_ascii(&self, filename: &str) -> io::Result<()>;
    fn write_ascii_internal(&self, writer: &mut impl Write) -> io::Result<()>;
    pub fn read_ascii(&mut self, filename: &str) -> io::Result<()>;
    fn read_ascii_internal(&mut self, reader: &mut impl Read) -> io::Result<()>;
}
impl PartialEq for ScanGraph {
    fn eq(&self, other: &Self) -> bool;
}
impl Clone for ScanGraph {
    fn clone(&self) -> Self;
}
```

## OcTree/OcTreeDataNode.rs
**crate::OcTree::DataNode**
```rs
pub struct OcTreeDataNode<T> {
    pub children: Option<Vec<Option<Box<OcTreeDataNode<T>>>>>,
    pub value: T,
}
impl<T> OcTreeDataNode<T>
where
    T: std::default::Default + Clone + std::cmp::PartialEq,
{
    pub fn new() -> Self;
    pub fn with_value(init_val: T) -> Self;
    pub fn clone(&self) -> Self;
    pub fn copy_data(&mut self, from: &OcTreeDataNode<T>);
    pub fn equals(&self, other: &OcTreeDataNode<T>) -> bool;
    fn alloc_children(&mut self);
    pub fn child_exists(&self, i: usize) -> bool;
    pub fn has_children(&self) -> bool;
    pub fn read_data(&mut self, s: &mut dyn std::io::Read) -> std::io::Result<()>;
    pub fn write_data(&self, s: &mut dyn std::io::Write) -> std::io::Result<()>;
}
impl<T> Drop for OcTreeDataNode<T> {
    fn drop(&mut self);
}
```

## OcTree/OcTreeNode.rs
**OcTree::Node**
```rs
pub struct OcTreeNode {
    pub data_node: DataNode<f32>,
}
impl OcTreeNode {
    pub fn new() -> Self;
    pub fn get_occupancy(&self) -> f64;
    pub fn get_log_odds(&self) -> f32;
    pub fn set_log_odds(&mut self, log_odds: f32);
    pub fn get_mean_child_log_odds(&self) -> f64;
    pub fn get_max_child_log_odds(&self) -> f32;
    pub fn update_occupancy_children(&mut self);
    pub fn add_value(&mut self, log_odds: f32);
}
```

## OcTree/OcTreeKey.rs
**OcTree::Key**
```rs
pub fn compute_child_key(pos: u8, center_offset_key: KeyType, parent_key: &OcTreeKey) -> OcTreeKey;
pub fn compute_child_idx(key: &OcTreeKey, depth: u8) -> u8;
pub fn compute_index_key(level: OcTreeKeyType, key: &OcTreeKey) -> OcTreeKey;

pub struct OcTreeKey {
    pub k: [OcTreeKeyType; 3],
}

impl OcTreeKey {
    pub fn new() -> Self;
    pub fn from_xyz(a: OcTreeKeyType, b: OcTreeKeyType, c: OcTreeKeyType) -> Self;
    pub fn eq(&self, other: &Self) -> bool;
    pub fn ne(&self, other: &Self) -> bool;
    pub fn get(&self, i: usize) -> OcTreeKeyType;
    pub fn get_mut(&mut self, i: usize) -> &mut OcTreeKeyType;
}

impl std::hash::Hash for OcTreeKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H);
}

impl PartialEq for OcTreeKey {
    fn eq(&self, other: &Self) -> bool;
}

impl Eq for OcTreeKey {}

impl Index<usize> for OcTreeKey {
    type Output = KeyType;
    fn index(&self, i: usize) -> &Self::Output;
}

impl IndexMut<usize> for OcTreeKey {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output;
}
```
**OcTree::KeyRay**
```rs
pub struct OcTreeKeyRay {
    pub ray: Vec<OcTreeKey>,
    pub end_of_ray: usize,
    pub max_size: usize,
}

impl OcTreeKeyRay {
    pub fn new() -> Self;
    pub fn reset(&mut self);
    pub fn add_key(&mut self, k: OcTreeKey);
    pub fn size(&self) -> usize;
    pub fn size_max(&self) -> usize;
}
```

## OcTree/
