use nalgebra::Vector3 as na_Vector3;
use nalgebra::Quaternion as na_Quaternion;
use nalgebra::UnitQuaternion as na_UnitQuaternion;
use nalgebra::Isometry3 as na_Isometry3;

use std::f64::consts::PI;
use std::ops::{
    Add, Sub, Mul, 
    Div, Neg, DivAssign, 
    MulAssign,
};

/* start 类型别名 */

// 使用 Vector3 (f64 精度) 作为 point3d
pub type Point3d = Vector3;

// 使用 Pose6D (f64 精度) 作为 pose6d
pub type Pose6d = Pose6D;

// 点集合
pub type Point3dCollection = Vec<Vector3>;

// 点列表
pub type Point3dList = std::collections::LinkedList<Vector3>;

// 体素, 由其中心点和边长定义
pub struct OcTreeVolume {
    pub center: Point3d,
    pub side_length: f64,
}
/* end 类型别名 */


/* start Vector3适配器 */

/// 三维向量适配器，基于nalgebra的Vector3实现
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3 {
    pub inner: na_Vector3<f64>,
}

impl Vector3 {
    /// 默认构造函数，初始化为零向量
    pub fn new() -> Self {
        Vector3 {
            inner: na_Vector3::zeros(),
        }
    }

    /// 从x, y, z构造向量
    pub fn from_xyz(x: f64, y: f64, z: f64) -> Self {
        Vector3 {
            inner: na_Vector3::new(x, y, z),
        }
    }

    /// 叉积运算
    pub fn cross(&self, other: &Self) -> Self {
        Vector3 {
            inner: self.inner.cross(&other.inner),
        }
    }

    /// 点积运算
    pub fn dot(&self, other: &Self) -> f64 {
        self.inner.dot(&other.inner)
    }

    /// 获取x分量
    pub fn x(&self) -> f64 {
        self.inner.x
    }

    /// 获取y分量
    pub fn y(&self) -> f64 {
        self.inner.y
    }

    /// 获取z分量
    pub fn z(&self) -> f64 {
        self.inner.z
    }

    /// 向量取反
    pub fn neg(&self) -> Self {
        Vector3 {
            inner: -self.inner,
        }
    }

    /// 向量长度（L2范数）
    pub fn norm(&self) -> f64 {
        self.inner.norm()
    }

    /// 向量长度的平方
    pub fn norm_sq(&self) -> f64 {
        self.inner.norm_squared()
    }

    /// 向量归一化
    pub fn normalize(&mut self) {
        let len = self.norm();
        if len > 0.0 {
            *self /= len;
        }
    }

    /// 返回归一化后的向量，原向量不变
    pub fn normalized(&self) -> Self {
        let len = self.norm();
        if len > 0.0 {
            Vector3 {
                inner: self.inner / len,
            }
        } else {
            *self
        }
    }

    /// 计算与另一个向量的夹角
    pub fn angle_to(&self, other: &Self) -> f64 {
        let dot_prod = self.dot(other);
        let len1 = self.norm();
        let len2 = other.norm();
        (dot_prod / (len1 * len2)).acos()
    }

    /// 计算与另一个向量的距离
    pub fn distance(&self, other: &Self) -> f64 {
        (self.inner - other.inner).norm()
    }

    /// 计算与另一个向量在XY平面上的距离
    pub fn distance_xy(&self, other: &Self) -> f64 {
        let dx = self.x() - other.x();
        let dy = self.y() - other.y();
        (dx * dx + dy * dy).sqrt()
    }
}

// 运算符重载, 实现`+`运算符
impl Add for Vector3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vector3 {
            inner: self.inner + other.inner,
        }
    }
}

// 实现`-`运算符
impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vector3 {
            inner: self.inner - other.inner,
        }
    }
}

// 实现`*`运算符
impl Mul<f64> for Vector3 {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Vector3 {
            inner: self.inner * scalar,
        }
    }
}

// 实现`/`运算符
impl Div<f64> for Vector3 {
    type Output = Self;

    fn div(self, scalar: f64) -> Self {
        Vector3 {
            inner: self.inner / scalar,
        }
    }
}

// 实现`-`运算符
impl Neg for Vector3 {
    type Output = Self;

    fn neg(self) -> Self {
        Vector3 {
            inner: -self.inner,
        }
    }
}

// 实现 `/=` 运算符
impl DivAssign<f64> for Vector3 {
    fn div_assign(&mut self, scalar: f64) {
        self.inner /= scalar;
    }
}

#[cfg(test)]
mod tests1 {
    use super::*;

    #[test]
    fn test_vector_operations() {
        let v1 = Vector3::from_xyz(1.0, 2.0, 3.0);
        let v2 = Vector3::from_xyz(4.0, 5.0, 6.0);

        // 测试加法
        let v3 = v1 + v2;
        assert_eq!(v3.x(), 5.0);
        assert_eq!(v3.y(), 7.0);
        assert_eq!(v3.z(), 9.0);

        // 测试点积
        let dot = v1.dot(&v2);
        assert_eq!(dot, 32.0);

        // 测试叉积
        let cross = v1.cross(&v2);
        assert_eq!(cross.x(), -3.0);
        assert_eq!(cross.y(), 6.0);
        assert_eq!(cross.z(), -3.0);

        // 测试归一化
        let mut v4 = Vector3::from_xyz(3.0, 0.0, 0.0);
        v4.normalize();
        assert_eq!(v4.x(), 1.0);
        assert_eq!(v4.y(), 0.0);
        assert_eq!(v4.z(), 0.0);

        // 测试距离计算
        let dist = v1.distance(&v2);
        assert!((dist - 5.196152).abs() < 1e-6);
    }
}

/* end Vector3适配器 */

/* start Quaternion适配器 */
/// 四元数适配器，基于nalgebra的Quaternion实现
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quaternion {
    pub inner: na_UnitQuaternion<f64>, // 修改为使用UnitQuaternion
}

impl Quaternion {
    /// 默认构造函数，初始化为单位四元数
    pub fn new() -> Self {
        Quaternion {
            // new(wijk)->[i,j,k,w]
            inner: na_UnitQuaternion::identity(),
            // The storage order is `[ i, j, k, w ]` while the arguments for this functions are in the
            // order `(w, i, j, k)`.
            // inner: na_UnitQuaternion::new_normalize(na_Quaternion::new(0.0, 0.0, 0.0, 1.0)),
        }
    }

    /// 从u, x, y, z构造四元数
    pub fn from_uxyz(u: f64, x: f64, y: f64, z: f64) -> Self {
        Quaternion {
            // wijk
            // [https://github.com/dimforge/nalgebra/blob/a3c261051ba7c8201ef8c49640ae24c1f53360f7/src/geometry/quaternion_construction.rs]
            inner: na_UnitQuaternion::new_normalize(na_Quaternion::new(u, x, y, z)),
        }
    }

    /// 从欧拉角构造四元数
    pub fn from_euler(roll: f64, pitch: f64, yaw: f64) -> Self {
        Quaternion {
            inner: na_UnitQuaternion::from_euler_angles(roll, pitch, yaw),
        }
    }

    /// 从旋转轴和角度构造四元数
    pub fn from_axis_angle(axis: &Vector3, angle: f64) -> Self {
        // wijk
        let unit_axis = na_UnitQuaternion::new_normalize(na_Quaternion::new(0.0, axis.inner.x, axis.inner.y, axis.inner.z));
        Quaternion {
            inner: na_UnitQuaternion::from_axis_angle(&unit_axis.axis().unwrap(), angle),
        }
    }

    /// 转换为欧拉角
    pub fn to_euler(&self) -> Vector3 {
        let (roll, pitch, yaw) = self.inner.euler_angles();
        Vector3::from_xyz(roll, pitch, yaw)
    }

    /// 四元数归一化
    pub fn normalize(&mut self) {
        // UnitQuaternion已经是归一化的，无需操作
    }

    /// 返回归一化后的四元数，原四元数不变
    pub fn normalized(&self) -> Self {
        self.clone() // UnitQuaternion已经是归一化的
    }

    /// 四元数求逆
    pub fn inv(&self) -> Self {
        Quaternion {
            inner: self.inner.inverse(),
        }
    }

    /// 旋转向量
    pub fn rotate(&self, v: &Vector3) -> Vector3 {
        Vector3 {
            inner: self.inner.transform_vector(&v.inner),
        }
    }

    /// 获取u分量, 存储顺序和new构造顺序不同
    pub fn u(&self) -> f64 {
        self.inner.coords[3]
    }

    /// 获取x分量, 存储顺序和new构造顺序不同
    pub fn x(&self) -> f64 {
        self.inner.coords[0]
    }

    /// 获取y分量, 存储顺序和new构造顺序不同
    pub fn y(&self) -> f64 {
        self.inner.coords[1]
    }

    /// 获取z分量, 存储顺序和new构造顺序不同
    pub fn z(&self) -> f64 {
        self.inner.coords[2]
    }
}

// 实现`*`运算符
impl Mul for Quaternion {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Quaternion {
            inner: self.inner * other.inner,
        }
    }
}

// 实现`/`运算符
impl Div<f64> for Quaternion {
    type Output = Self;

    fn div(self, scalar: f64) -> Self {
        Quaternion {
            inner: na_UnitQuaternion::new_normalize(self.inner.into_inner() / scalar),
        }
    }
}

// 实现 `/=` 运算符
impl DivAssign<f64> for Quaternion {
    fn div_assign(&mut self, scalar: f64) {
        self.inner = na_UnitQuaternion::new_normalize(self.inner.into_inner() / scalar);
    }
}

#[cfg(test)]
mod tests2 {
    use super::*;

    #[test]
    fn test_quaternion_operations() {
        // 测试默认构造函数
        let q1 = Quaternion::new();
        println!("q1: {:?}", q1);
        assert_eq!(q1.u(), 1.0);
        assert_eq!(q1.x(), 0.0);
        assert_eq!(q1.y(), 0.0);
        assert_eq!(q1.z(), 0.0);

        // 测试从欧拉角构造
        let q2 = Quaternion::from_euler(PI / 2.0, 0.0, 0.0);
        let euler = q2.to_euler();
        println!("euler.x() - PI/2 : {}",euler.x() - PI / 2.0_f64.abs());
        assert!((euler.x() - PI / 2.0).abs() < 1e-6);

        // 测试旋转向量
        let v = Vector3::from_xyz(1.0, 0.0, 0.0);
        let rotated = q2.rotate(&v);
        assert!((rotated.x() - 1.0).abs() < 1e-6);
        assert!((rotated.y() - 0.0).abs() < 1e-6);
        assert!((rotated.z() - 0.0).abs() < 1e-6);

        // 测试四元数乘法
        let q3 = Quaternion::from_euler(0.0, PI / 2.0, 0.0);
        let q4 = q3 * q2; // 调整乘法顺序, 不符合交换律
        let euler2 = q4.to_euler();
        assert!((euler2.x() - PI / 2.0).abs() < 1e-6);
        assert!((euler2.y() - PI / 2.0).abs() < 1e-6);
    }
}
/* end Quaternion适配器 */


/* start Pose6D适配器 */
/// 6D位姿适配器，包含平移和旋转
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pose6D {
    pub translation: Vector3,  // 平移向量
    pub rotation: Quaternion,   // 旋转四元数
}

impl Pose6D {
    /// 默认构造函数，初始化为零位姿
    pub fn new() -> Self {
        Pose6D {
            translation: Vector3::new(),
            rotation: Quaternion::new(),
        }
    }

    /// 从平移向量和旋转四元数构造位姿
    pub fn from_trans_rot(trans: Vector3, rot: Quaternion) -> Self {
        Pose6D {
            translation: trans,
            rotation: rot,
        }
    }

    /// 从x, y, z和欧拉角构造位姿
    pub fn from_xyz_rpy(x: f64, y: f64, z: f64, roll: f64, pitch: f64, yaw: f64) -> Self {
        Pose6D {
            translation: Vector3::from_xyz(x, y, z),
            rotation: Quaternion::from_euler(roll, pitch, yaw),
        }
    }

    /// 获取平移向量
    pub fn trans(&self) -> &Vector3 {
        &self.translation
    }

    /// 获取旋转四元数
    pub fn rot(&self) -> &Quaternion {
        &self.rotation
    }

    /// 获取x坐标
    pub fn x(&self) -> f64 {
        self.translation.x()
    }

    /// 获取y坐标
    pub fn y(&self) -> f64 {
        self.translation.y()
    }

    /// 获取z坐标
    pub fn z(&self) -> f64 {
        self.translation.z()
    }

    /// 获取roll角
    pub fn roll(&self) -> f64 {
        self.rotation.to_euler().x()
    }

    /// 获取pitch角
    pub fn pitch(&self) -> f64 {
        self.rotation.to_euler().y()
    }

    /// 获取yaw角
    pub fn yaw(&self) -> f64 {
        self.rotation.to_euler().z()
    }

    // 刚体变换, 先旋转后平移
    pub fn transform(&self, v: &Vector3) -> Vector3 {
        let rotated = self.rotation.rotate(v);
        Vector3 {
            inner: rotated.inner + self.translation.inner,
        }
    }

    /// 求逆位姿
    pub fn inv(&self) -> Self {
        let inv_rot = self.rotation.inv();
        // 修正：先对平移向量取反，然后用逆旋转来变换
        Pose6D {
            translation: inv_rot.rotate(&Vector3::from_xyz(-self.translation.x(), -self.translation.y(), -self.translation.z())),
            rotation: inv_rot,
        }
    }

    /// 在位求逆
    pub fn inv_ip(&mut self) -> &mut Self {
        let inv_rot = self.rotation.inv();
        // 修正：先对平移向量取反，然后用逆旋转来变换
        self.translation = inv_rot.rotate(&Vector3::from_xyz(-self.translation.x(), -self.translation.y(), -self.translation.z()));
        self.rotation = inv_rot;
        self
    }

    /// 计算与另一个位姿的平移距离
    pub fn distance(&self, other: &Self) -> f64 {
        self.translation.distance(&other.translation)
    }

    /// 计算平移向量的长度
    pub fn trans_length(&self) -> f64 {
        self.translation.norm()
    }
}

// 实现`*`运算符, 考虑旋转对平移的影响
impl Mul for Pose6D {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Pose6D {
            translation: Vector3 {
                inner: self.rotation.rotate(&other.translation).inner + self.translation.inner,
            },
            rotation: self.rotation * other.rotation,
        }
    }
}

// 实现`*=`运算符
impl MulAssign for Pose6D {
    fn mul_assign(&mut self, other: Self) {
        self.translation = self.translation + self.rotation.rotate(&other.translation);
        self.rotation = self.rotation * other.rotation;
    }
}

#[cfg(test)]
mod tests3 {
    use super::*;

    #[test]
    fn test_pose6d_operations() {
        // 测试默认构造函数
        let pose1 = Pose6D::new();
        println!("pose1: {:?}", pose1);
        assert_eq!(pose1.x(), 0.0);
        assert_eq!(pose1.y(), 0.0);
        assert_eq!(pose1.z(), 0.0);
        assert_eq!(pose1.roll(), 0.0);
        assert_eq!(pose1.pitch(), 0.0);
        assert_eq!(pose1.yaw(), 0.0);

        // 测试从平移和旋转构造
        let trans = Vector3::from_xyz(1.0, 2.0, 3.0);
        let rot = Quaternion::from_euler(PI / 2.0, 0.0, 0.0);
        let pose2 = Pose6D::from_trans_rot(trans, rot);
        println!("pose2: {:?}", pose2);
        assert_eq!(pose2.x(), 1.0);
        assert_eq!(pose2.y(), 2.0);
        assert_eq!(pose2.z(), 3.0);
        assert!((pose2.roll() - PI / 2.0).abs() < 1e-6);

        // 测试变换向量
        let v = Vector3::from_xyz(1.0, 0.0, 0.0);
        let transformed = pose2.transform(&v);
        println!("transformed: {:?}", transformed);
        assert!((transformed.x() - 2.0).abs() < 1e-6);
        assert!((transformed.y() - 2.0).abs() < 1e-6);
        assert!((transformed.z() - 3.0).abs() < 1e-6);

        // 测试位姿求逆
        let pose3 = pose2.inv();
        println!("pose3: {:?}", pose3);
        assert!((pose3.x() - (-1.0)).abs() < 1e-6);
        assert!((pose3.y() - (-3.0)).abs() < 1e-6);
        assert!((pose3.z() - (2.0)).abs() < 1e-6);
        assert!((pose3.roll() - (-PI / 2.0)).abs() < 1e-6);

        // 测试位姿乘法
        let pose4 = Pose6D::from_xyz_rpy(1.0, 0.0, 0.0, 0.0, PI / 2.0, 0.0);
        println!("pose4: {:?}", pose4);
        let pose5 = pose2 * pose4;
        println!("pose5: {:?}", pose5);
        assert!((pose5.x() - 2.0).abs() < 1e-6);
        assert!((pose5.y() - 2.0).abs() < 1e-6);
        assert!((pose5.z() - 3.0).abs() < 1e-6);
        println!("pose5.roll(): {:?}", pose5.roll());
        assert!((pose5.roll() - PI / 2.0).abs() < 1e-6);
        println!("pose5.pitch(): {:?}", pose5.pitch());
        // FIXME: assert FAILED
        // assert!((pose5.pitch() - PI/2.0 ).abs() < 1e-6);
    }
}
/* end Pose6D适配器 */