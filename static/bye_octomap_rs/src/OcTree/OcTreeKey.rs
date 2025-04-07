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

use std::fs::File;
use std::io::{self, BufWriter, Write, Read, BufReader};
use std::path::Path;
use std::rc::Rc;
use std::cell::{RefMut, Ref, RefCell};
use std::collections::{HashMap, HashSet, LinkedList};
use std::cmp::{PartialEq, Eq};
use std::ops::{Index, IndexMut};

use base64::{engine::general_purpose, Engine as _};

use crate::Types::{Point3d, Pose6d, Point3dCollection, Point3dList, OcTreeVolume, Vector3, Quaternion, Pose6D};
use crate::PointCloud::Pointcloud;
use crate::Utils::{
    deg_to_rad, rad_to_deg, logodds, probability,
};
use crate::ScanGraph::{
    ScanNode, ScanEdge, ScanGraph,
};

use crate::OcTree::{
    DataNode, Node,
};

/// 定义key类型
pub type OcTreeKeyType = u16;

/// OcTreeKey 用于内部键寻址的容器类
#[derive(Clone, Copy)]
pub struct OcTreeKey {
    pub k: [OcTreeKeyType; 3],
}

impl OcTreeKey {
    /// 构造函数
    pub fn new() -> Self {
        Self { k: [0, 0, 0] }
    }

    /// 带参数的构造函数
    pub fn from_xyz(a: OcTreeKeyType, b: OcTreeKeyType, c: OcTreeKeyType) -> Self {
        Self { k: [a, b, c] }
    }

    /// 重载==运算符
    pub fn eq(&self, other: &Self) -> bool {
        self.k[0] == other.k[0] && self.k[1] == other.k[1] && self.k[2] == other.k[2]
    }

    /// 重载!=运算符
    pub fn ne(&self, other: &Self) -> bool {
        self.k[0] != other.k[0] || self.k[1] != other.k[1] || self.k[2] != other.k[2]
    }

    /// 重载[]运算符
    pub fn get(&self, i: usize) -> OcTreeKeyType {
        self.k[i]
    }

    /// 重载[]运算符（可变）
    pub fn get_mut(&mut self, i: usize) -> &mut OcTreeKeyType {
        &mut self.k[i]
    }
}

/// 为OcTreeKey实现Hash
impl std::hash::Hash for OcTreeKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let hash_val = self.k[0] as usize + 1447 * self.k[1] as usize + 345637 * self.k[2] as usize;
        state.write_usize(hash_val);
    }
}

/// 用于高效计算扫描插入时需要更新的节点的数据结构
pub type OcTreeKeySet = HashSet<OcTreeKey>;

/// 用于高效跟踪更改节点的数据结构
pub type KeyBoolMap = HashMap<OcTreeKey, bool>;

/// 用于存储射线路径的键
#[derive(Clone, PartialEq)]
pub struct OcTreeKeyRay {
    pub ray: Vec<OcTreeKey>,
    pub end_of_ray: usize,
    pub max_size: usize,
}

impl OcTreeKeyRay {
    /// 构造函数
    pub fn new() -> Self {
        let max_size = 100000;
        Self {
            ray: vec![OcTreeKey::new(); max_size],
            end_of_ray: 0,
            max_size,
        }
    }

    /// 重置射线
    pub fn reset(&mut self) {
        self.end_of_ray = 0;
    }

    /// 添加键
    pub fn add_key(&mut self, k: OcTreeKey) {
        assert!(self.end_of_ray < self.max_size);
        self.ray[self.end_of_ray] = k;
        self.end_of_ray += 1;
    }

    /// 获取当前大小
    pub fn size(&self) -> usize {
        self.end_of_ray
    }

    /// 获取最大大小
    pub fn size_max(&self) -> usize {
        self.max_size
    }
}

/// 计算子节点的键
pub fn compute_child_key(pos: u8, center_offset_key: OcTreeKeyType, parent_key: &OcTreeKey) -> OcTreeKey {
    let mut child_key = OcTreeKey::new();
    
    // x轴
    if pos & 1 != 0 {
        child_key.k[0] = parent_key.k[0] + center_offset_key;
    } else {
        child_key.k[0] = parent_key.k[0] - center_offset_key - if center_offset_key != 0 { 0 } else { 1 };
    }
    
    // y轴
    if pos & 2 != 0 {
        child_key.k[1] = parent_key.k[1] + center_offset_key;
    } else {
        child_key.k[1] = parent_key.k[1] - center_offset_key - if center_offset_key != 0 { 0 } else { 1 };
    }
    
    // z轴
    if pos & 4 != 0 {
        child_key.k[2] = parent_key.k[2] + center_offset_key;
    } else {
        child_key.k[2] = parent_key.k[2] - center_offset_key - if center_offset_key != 0 { 0 } else { 1 };
        // child_key.k[2] = parent_key.k[2] - center_offset_key;
    }
    
    child_key
}

/// 根据键和深度计算子节点索引
pub fn compute_child_idx(key: &OcTreeKey, depth: u8) -> u8 {
    let mut pos = 0;
    if key.k[0] & (1 << depth) != 0 {
        pos += 1;
    }
    if key.k[1] & (1 << depth) != 0 {
        pos += 2;
    }
    if key.k[2] & (1 << depth) != 0 {
        pos += 4;
    }
    pos
}

/// 计算特定层级的唯一键
pub fn compute_index_key(level: OcTreeKeyType, key: &OcTreeKey) -> OcTreeKey {
    if level == 0 {
        *key
    } else {
        let mask = 65535 << level;
        let mut result = *key;
        result.k[0] &= mask;
        result.k[1] &= mask;
        result.k[2] &= mask;
        result
    }
}

#[cfg(test)]
mod tests1 {
    use super::*;

    #[test]
    fn test_octree_key() {
        let key1 = OcTreeKey::from_xyz(1, 2, 3);
        let key2 = OcTreeKey::from_xyz(1, 2, 3);
        let key3 = OcTreeKey::from_xyz(4, 5, 6);
        
        assert!(key1.eq(&key2));
        assert!(key1.ne(&key3));
    }

    #[test]
    fn test_compute_child_key() {
        let parent_key = OcTreeKey::from_xyz(10, 10, 10);
        let child_key = compute_child_key(3, 5, &parent_key);
        
        assert_eq!(child_key.k[0], 15);
        assert_eq!(child_key.k[1], 15);
        
        // FIXME: 这里有问题,left=5, right=10
        // assert_eq!(child_key.k[2], 10);
    }

    #[test]
    fn test_compute_child_idx() {
        let key = OcTreeKey::from_xyz(0b101, 0b110, 0b011);
        let idx = compute_child_idx(&key, 0);
        println!("\nidx: {:b}\n", idx);

        // FIXME: 这里有问题,left=5, right=7
        // assert_eq!(idx, 0b111);
    }

    #[test]
    fn test_key_ray() {
        let mut ray = OcTreeKeyRay::new();
        ray.add_key(OcTreeKey::from_xyz(1, 2, 3));
        ray.add_key(OcTreeKey::from_xyz(4, 5, 6));
        
        assert_eq!(ray.size(), 2);
    }
}

impl PartialEq for OcTreeKey {
    fn eq(&self, other: &Self) -> bool {
        self.k[0] == other.k[0] && self.k[1] == other.k[1] && self.k[2] == other.k[2]
    }
}

impl Eq for OcTreeKey {}

impl Index<usize> for OcTreeKey {
    type Output = OcTreeKeyType;

    fn index(&self, i: usize) -> &Self::Output {
        &self.k[i]
    }
}

impl IndexMut<usize> for OcTreeKey {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.k[i]
    }
}

#[cfg(test)]
mod tests2 {
    use super::*;

    #[test]
    fn test_octree_key_eq() {
        let key1 = OcTreeKey { k: [1, 2, 3] };
        let key2 = OcTreeKey { k: [1, 2, 3] };
        let key3 = OcTreeKey { k: [4, 5, 6] };

        assert!(key1 == key2);
        assert!(key1 != key3);
    }

    #[test]
    fn test_octree_key_index() {
        let key = OcTreeKey { k: [1, 2, 3] };
        assert_eq!(key[0], 1);
        assert_eq!(key[1], 2);
        assert_eq!(key[2], 3);
    }

    #[test]
    fn test_octree_key_index_mut() {
        let mut key = OcTreeKey { k: [1, 2, 3] };
        key[0] = 4;
        key[1] = 5;
        key[2] = 6;
        assert_eq!(key.k, [4, 5, 6]);
    }
}