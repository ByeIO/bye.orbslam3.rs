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

//! 使用oktree库的Octree替代cpp代码中的OcTree

use std::collections::LinkedList;
use std::fs::File;
use std::io::{self, BufWriter, Write, Read, BufReader};
use std::path::Path;
use std::rc::Rc;
use std::cell::{RefMut, Ref, RefCell};
use base64::{engine::general_purpose, Engine as _};

// 注意: oktree库使用u128类型进行计算
use oktree::{
    bounding::{Aabb, TUVec3, Unsigned, TUVec3u128},
    node::{Branch, Node, NodeType},
    pool::{Pool, PoolElementIterator, PoolIntoIterator, /* PoolItem, */ PoolIterator, PoolIteratorMut},
    ElementId, NodeId, TreeError, Volume,
    tree::Octree, Position, 
};
use nalgebra::{
    Matrix3, UnitQuaternion, 
    Isometry3, Translation3, Const,
    Matrix, Point3, ViewStorage, Rotation3,
    Matrix3x1, VectorView3, DMatrix, DVector, SVector,
    Vector2, Matrix2, 
};
use nalgebra::Vector3 as na_Vector3;
use nalgebra::Quaternion as na_Quaternion;

use crate::Types::{Point3d, Pose6d, Point3dCollection, Point3dList, OcTreeVolume, Vector3, Quaternion, Pose6D};
use crate::PointCloud::Pointcloud;
use crate::Utils::{
    deg_to_rad, rad_to_deg, logodds, probability,
};
use crate::ScanGraph::{
    ScanNode, ScanEdge, ScanGraph,
};

/* start 兼容oktree类型trait */
#[derive(Clone)]
pub struct Vector3Wrapper {
    inner: Vector3,
}

impl Position for Vector3Wrapper {
    type U = u128;
    fn position(&self) -> TUVec3<u128> {
        TUVec3::new(
            OkTree::f64_to_u128(self.inner.x()),
            OkTree::f64_to_u128(self.inner.y()),
            OkTree::f64_to_u128(self.inner.z()),
        )
    }
}

// impl Volume for Vector3Wrapper {
//     type U = u128;
//     fn volume(&self) -> Aabb<u128> {
//         let point = self.position();
//         Aabb::new_unchecked(point, OkTree::f64_to_u128(0.0)) // 假设体积为 0
//     }
// }
/* end 兼容oktree类型trait */

pub struct OkTree {
    // 分辨率
    pub resolution: f64,
    // 八叉树实例
    octree: Octree<u128, Vector3Wrapper>,
}

impl OkTree{
    // 构造函数
    pub fn new(resolution: f64) -> Self {
        // 将分辨率转换为u128类型
        let res_u128 = Self::f64_to_u128(resolution);
        
        // 创建AABB边界框，假设地图范围为1000米
        let half_size = Self::f64_to_u128(500.0);
        let center = TUVec3u128::new(half_size, half_size, half_size);
        let aabb = Aabb::new_unchecked(center.0, half_size);

        // 初始化八叉树
        let octree = Octree::from_aabb(aabb);

        Self{
            resolution,
            octree,
        }
    }

    // 将点云存入八叉树地图，给定原点，这样可以计算投射线
    pub fn insert_pointcloud(&mut self, cloud: Pointcloud, origin: na_Vector3<f64>) {
        // 将原点转换为u128类型
        let origin_u128 = TUVec3u128::new(
            OkTree::f64_to_u128(origin.x),
            OkTree::f64_to_u128(origin.y),
            OkTree::f64_to_u128(origin.z),
        );
    
        // 遍历点云中的每个点
        for point in cloud.points {
            // 将点包装为 Vector3Wrapper
            let point_wrapper = Vector3Wrapper { inner: point };
    
            // 插入点到八叉树中
            self.octree.insert(point_wrapper).unwrap();
        }
    }

    // 更新中间节点的占据信息
    pub fn update_inner_occupancy(&mut self) {
        // 遍历八叉树的所有节点
        for node in self.octree.iter_nodes() {
            if let NodeType::Branch(branch) = &node.ntype {
                // 计算子节点的占据概率
                let mut occupied_count = 0;
                let mut total_count = 0;
    
                for child_id in branch.children.iter() {
                    // if let Some(child) = self.octree.get_node(*child_id) {
                    if let Some(child) = self.octree.get_element(ElementId::from(*child_id)) {
                        total_count += 1;
                        // if let NodeType::Leaf(_) = child.ntype {
                        //     occupied_count += 1;
                        // }
                    }
                }
    
                // 更新当前节点的占据信息
                if total_count > 0 {
                    let occupancy = occupied_count as f64 / total_count as f64;
    
                    // TODO: 根据占据概率更新节点信息
                    if occupancy > 0.5 {
                        // 如果占据概率大于50%，标记为占据
                        // self.octree.update_node(node.id, NodeType::Leaf(ElementId(0))); // 使用占位符ElementId
                    } else {
                        // 否则标记为空闲
                        // self.octree.update_node(node.id, NodeType::Empty);
                    }
                }
            }
        }
    }

    // 写入磁盘
    pub fn write_binary(&mut self, s: &mut impl Write) -> io::Result<()> {
        // Serialize the octree into a byte vector
        let mut buffer = Vec::new();

        // TODO
        // self.octree.write_binary(&mut buffer)?;
    
        // Encode the binary data into base64
        let encoded = general_purpose::STANDARD.encode(&buffer);
    
        // Write the base64 string to the output
        s.write_all(encoded.as_bytes())?;
        Ok(())
    }

    // 工具函数: f64->u128, 误差1e-6
    pub fn f64_to_u128(value: f64) -> u128 {
        (value * 1_000_000.0).round() as u128
    }

    // 工具函数: u128->f64, 误差1e-6
    pub fn u128_to_f64(value: u128) -> f64 {
        value as f64 / 1_000_000.0
    }
}

#[cfg(test)]
mod tests1 {
    use super::*;
    use crate::PointCloud::Pointcloud;
    use nalgebra::Vector3;

    #[test]
    fn test_oktree_initialization() {
        let resolution = 0.1; // 10 cm resolution
        let octree = OkTree::new(resolution);

        assert!(octree.resolution - resolution < 1e-6);
        // assert!(!octree.octree.is_empty());
    }

    #[test]
    fn test_insert_pointcloud() {
        let mut octree = OkTree::new(0.1);

        let mut cloud = Pointcloud::new();
        cloud.push_back(1.0, 2.0, 3.0);
        cloud.push_back(4.0, 5.0, 6.0);

        let origin = Vector3::new(0.0, 0.0, 0.0);
        octree.insert_pointcloud(cloud, origin);

        // Verify that the points were inserted
        let point1 = TUVec3::new(
            OkTree::f64_to_u128(1.0),
            OkTree::f64_to_u128(2.0),
            OkTree::f64_to_u128(3.0),
        );
        let point2 = TUVec3::new(
            OkTree::f64_to_u128(4.0),
            OkTree::f64_to_u128(5.0),
            OkTree::f64_to_u128(6.0),
        );

        assert!(octree.octree.find(&point1).is_some());
        assert!(octree.octree.find(&point2).is_some());
    }

    #[test]
    fn test_write_binary() {
        let mut octree = OkTree::new(0.1);

        let mut cloud = Pointcloud::new();
        cloud.push_back(1.0, 2.0, 3.0);
        cloud.push_back(4.0, 5.0, 6.0);

        let origin = Vector3::new(0.0, 0.0, 0.0);
        octree.insert_pointcloud(cloud, origin);

        // Write to a buffer
        let mut buffer = Vec::new();
        octree.write_binary(&mut buffer).unwrap();

        // Verify that the output is a valid base64 string
        let encoded = String::from_utf8(buffer).unwrap();
        assert!(general_purpose::STANDARD.decode(&encoded).is_ok());
    }

    #[test]
    fn test_f64_to_u128_conversion() {
        let value = 123.456789;
        let converted = OkTree::f64_to_u128(value);
        let restored = OkTree::u128_to_f64(converted);

        // Check if the restored value is within the expected precision
        assert!((restored - value).abs() < 1e-6);
    }

    #[test]
    fn test_u128_to_f64_conversion() {
        let value = 123456789;
        let converted = OkTree::u128_to_f64(value);
        let restored = OkTree::f64_to_u128(converted);

        // Check if the restored value matches the original
        assert_eq!(restored, value);
    }
}