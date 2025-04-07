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

// 使用标准库和给定的类型定义
use std::collections::LinkedList;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::Path;

use rand::{
    prelude::IndexedRandom,
    rng, 
};

// 使用给定的类型别名和结构体
use crate::Types::{
    Point3d, Pose6d, Point3dCollection, 
    Point3dList, OcTreeVolume, Vector3, Quaternion, Pose6D
};

/// Pointcloud 结构体，表示3D点云
#[derive(Clone, Debug)]
pub struct Pointcloud {
    // 点云中的点集合
    pub points: Point3dCollection, 
    // 当前逆变换
    pub current_inv_transform: Pose6d, 
}

impl Pointcloud {
    /// 创建一个新的空点云
    pub fn new() -> Self {
        Pointcloud {
            points: Vec::new(),
            current_inv_transform: Pose6D::new(),
        }
    }

    /// 清空点云中的所有点
    pub fn clear(&mut self) {
        self.points.clear();
    }

    /// 获取点云中的点数
    pub fn size(&self) -> usize {
        self.points.len()
    }

    /// 预留点云的空间
    pub fn reserve(&mut self, size: usize) {
        self.points.reserve(size);
    }

    /// 向点云中添加一个点
    pub fn push_back(&mut self, x: f64, y: f64, z: f64) {
        self.points.push(Vector3::from_xyz(x, y, z));
    }

    /// 向点云中添加一个点（通过引用）
    pub fn push_back_ref(&mut self, p: &Point3d) {
        self.points.push(p.clone());
    }

    /// 向点云中添加另一个点云的所有点
    pub fn push_back_cloud(&mut self, other: &Pointcloud) {
        for point in &other.points {
            self.points.push(point.clone());
        }
    }

    /// 对点云中的每个点应用变换
    pub fn transform(&mut self, transform: Pose6d) {
        for point in &mut self.points {
            *point = transform.transform(point);
        }
        self.current_inv_transform = transform.inv();
    }

    /// 对点云中的每个点应用绝对变换
    pub fn transform_absolute(&mut self, transform: Pose6d) {
        let transf = self.current_inv_transform * transform;
        for point in &mut self.points {
            *point = transf.transform(point);
        }
        self.current_inv_transform = transform.inv();
    }

    /// 对点云中的每个点进行旋转
    pub fn rotate(&mut self, roll: f64, pitch: f64, yaw: f64) {
        let quat = Quaternion::from_euler(roll, pitch, yaw);
        for point in &mut self.points {
            *point = quat.rotate(point);
        }
    }

    /// 计算点云的边界框
    pub fn calc_bbx(&self) -> (Point3d, Point3d) {
        let mut min_x = f64::MAX;
        let mut min_y = f64::MAX;
        let mut min_z = f64::MAX;
        let mut max_x = f64::MIN;
        let mut max_y = f64::MIN;
        let mut max_z = f64::MIN;

        for point in &self.points {
            let x = point.x();
            let y = point.y();
            let z = point.z();

            if x < min_x { min_x = x; }
            if y < min_y { min_y = y; }
            if z < min_z { min_z = z; }
            if x > max_x { max_x = x; }
            if y > max_y { max_y = y; }
            if z > max_z { max_z = z; }
        }

        (Vector3::from_xyz(min_x, min_y, min_z), Vector3::from_xyz(max_x, max_y, max_z))
    }

    /// 裁剪点云到给定的边界框
    pub fn crop(&mut self, lower_bound: Point3d, upper_bound: Point3d) {
        let mut result = Vec::new();
        for point in &self.points {
            let x = point.x();
            let y = point.y();
            let z = point.z();
            println!("\n当前裁剪点:({},{},{})...\n", x, y, z);

            // 在边界框内
            if x >= lower_bound.x() && x <= upper_bound.x() &&
               y >= lower_bound.y() && y <= upper_bound.y() &&
               z >= lower_bound.z() && z <= upper_bound.z() {
                println!("\n({},{},{})在边界框内...\n", x, y, z);
                result.push(point.clone());
            }
        }
        self.points = result;
    }

    /// 移除距离原点小于给定阈值的点
    pub fn min_dist(&mut self, thres: f64) {
        let mut result = Vec::new();
        for point in &self.points {
            let dist = point.norm();
            if dist > thres {
                result.push(point.clone());
            }
        }
        self.points = result;
    }

    /// 随机子采样点云
    pub fn sub_sample_random(&self, num_samples: usize) -> Pointcloud {
        let mut sample_cloud = Pointcloud::new();
        let mut rng = rand::rng();
        let samples: Vec<_> = self.points.choose_multiple(&mut rng, num_samples).cloned().collect();
        for sample in samples {
            sample_cloud.push_back_ref(&sample);
        }
        sample_cloud
    }

    /// 将点云写入VRML文件
    pub fn write_vrml(&self, filename: &Path) -> io::Result<()> {
        let mut file = BufWriter::new(File::create(filename)?);
        writeln!(file, "#VRML V2.0 utf8")?;
        writeln!(file, "Transform {{")?;
        writeln!(file, "translation 0 0 0")?;
        writeln!(file, "rotation 0 0 0 0")?;
        writeln!(file, "  children [")?;
        writeln!(file, "     Shape{{")?;
        writeln!(file, "  geometry PointSet {{")?;
        writeln!(file, "      coord Coordinate {{")?;
        writeln!(file, "          point [")?;

        for point in &self.points {
            writeln!(file, "\t\t{} {} {}", point.x(), point.y(), point.z())?;
        }

        writeln!(file, "                 ]")?;
        writeln!(file, "      }}")?;
        writeln!(file, "    color Color{{")?;
        writeln!(file, "              color [")?;

        for _ in &self.points {
            writeln!(file, "\t\t 1.0 1.0 1.0")?;
        }

        writeln!(file, "                 ]")?;
        writeln!(file, "      }}")?;
        writeln!(file, "   }}")?;
        writeln!(file, "     }}")?;
        writeln!(file, "  ]")?;
        writeln!(file, "}}")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests1 {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_pointcloud() {
        let mut cloud = Pointcloud::new();
        cloud.push_back(1.0, 2.0, 3.0);
        cloud.push_back(4.0, 5.0, 6.0);
        assert_eq!(cloud.size(), 2);

        let (lower, upper) = cloud.calc_bbx();
        assert_eq!(lower.x(), 1.0);
        assert_eq!(upper.z(), 6.0);

        cloud.crop(Vector3::from_xyz(1.0, 2.0, 2.0), Vector3::from_xyz(5.0, 5.0, 5.0));
        assert_eq!(cloud.size(), 1);

        cloud.write_vrml(Path::new("./target/test.wrl")).unwrap();
    }
}