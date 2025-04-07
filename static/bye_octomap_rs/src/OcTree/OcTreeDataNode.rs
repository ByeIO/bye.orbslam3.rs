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

use std::collections::LinkedList;
use std::fs::File;
use std::io::{self, BufWriter, Write, Read, BufReader};
use std::path::Path;
use std::rc::Rc;
use std::cell::{RefMut, Ref, RefCell};
use base64::{engine::general_purpose, Engine as _};

use crate::Types::{Point3d, Pose6d, Point3dCollection, Point3dList, OcTreeVolume, Vector3, Quaternion, Pose6D};
use crate::PointCloud::Pointcloud;
use crate::Utils::{
    deg_to_rad, rad_to_deg, logodds, probability,
};
use crate::ScanGraph::{
    ScanNode, ScanEdge, ScanGraph,
};

// 定义OcTreeDataNode结构体，用于表示八叉树中的节点
#[derive(Clone, PartialEq, Default)]
pub struct OcTreeDataNode<T> {
    pub children: Option<Vec<Option<Box<OcTreeDataNode<T> > > > >, // 子节点数组，可能为空
    pub value: T, // 节点存储的数据
}

impl<T> OcTreeDataNode<T>
where T: std::default::Default + Clone + std::cmp::PartialEq,
{
    // 构造函数，创建一个空的节点
    pub fn new() -> Self {
        OcTreeDataNode {
            children: None,
            value: Default::default(),
        }
    }

    // 带初始值的构造函数
    pub fn with_value(init_val: T) -> Self {
        OcTreeDataNode {
            children: None,
            value: init_val,
        }
    }

    // 复制构造函数，递归深拷贝所有子节点及其数据
    pub fn clone(&self) -> Self {
        let mut new_node = OcTreeDataNode {
            children: None,
            value: self.value.clone(),
        };

        if let Some(children) = &self.children {
            new_node.alloc_children();
            for i in 0..8 {
                if let Some(child) = &children[i] {
                    new_node.children.as_mut().unwrap()[i] = Some(Box::new(*child.clone()));
                }
            }
        }

        new_node
    }

    // 复制节点数据，不复制子节点
    pub fn copy_data(&mut self, from: &OcTreeDataNode<T>) {
        self.value = from.value.clone();
    }

    // 判断两个节点是否相等
    pub fn equals(&self, other: &OcTreeDataNode<T>) -> bool {
        self.value == other.value
    }

    // 分配子节点数组
    fn alloc_children(&mut self) {
        let mut children = Vec::with_capacity(8);
        for _ in 0..8 {
            children.push(None);
        }
        self.children = Some(children);
    }

    // 判断第i个子节点是否存在
    pub fn child_exists(&self, i: usize) -> bool {
        assert!(i < 8);
        if let Some(children) = &self.children {
            if let Some(child) = &children[i] {
                return true;
            }
        }
        false
    }

    // 判断节点是否有子节点
    pub fn has_children(&self) -> bool {
        if let Some(children) = &self.children {
            for i in 0..8 {
                if children[i].is_some() {
                    return true;
                }
            }
        }
        false
    }

    // 从二进制流中读取节点数据
    pub fn read_data(&mut self, s: &mut dyn std::io::Read) -> std::io::Result<()> {
        let size = std::mem::size_of::<T>();
        let mut buf = vec![0u8; size];
        s.read_exact(&mut buf)?;
        self.value = unsafe { std::ptr::read(buf.as_ptr() as *const T) };
        Ok(())
    }

    // 将节点数据写入二进制流
    pub fn write_data(&self, s: &mut dyn std::io::Write) -> std::io::Result<()> {
        let buf = unsafe {
            std::slice::from_raw_parts(
                &self.value as *const T as *const u8,
                std::mem::size_of::<T>(),
            )
        };
        s.write_all(buf)?;
        Ok(())
    }

}

// 实现Drop trait，确保节点析构时正确处理子节点
impl<T> Drop for OcTreeDataNode<T> {
    fn drop(&mut self) {
        // 确保子节点已经被正确释放
        // 移除断言，Rust 会自动处理 children 的析构
        // assert!(self.children.is_none());
    }
}

#[cfg(test)]
mod tests1 {
    use super::*;

    // 测试默认构造函数
    #[test]
    fn test_new() {
        let node: OcTreeDataNode<i32> = OcTreeDataNode::new();
        assert_eq!(node.value, 0); // 默认值应为0
        assert!(node.children.is_none()); // 子节点应为None
    }

    // 测试带初始值的构造函数
    #[test]
    fn test_with_value() {
        let node = OcTreeDataNode::with_value(42);
        assert_eq!(node.value, 42); // 值应为42
        assert!(node.children.is_none()); // 子节点应为None
    }

    // 测试复制构造函数
    #[test]
    fn test_clone() {
        let mut node = OcTreeDataNode::with_value(10);
        node.alloc_children(); // 分配子节点
        let cloned_node = node.clone();
        assert_eq!(cloned_node.value, 10); // 值应相同
        assert!(cloned_node.children.is_some()); // 子节点应存在
    }

    // 测试数据复制
    #[test]
    fn test_copy_data() {
        let mut node1 = OcTreeDataNode::with_value(5);
        let node2 = OcTreeDataNode::with_value(10);
        node1.copy_data(&node2);
        assert_eq!(node1.value, 10); // 值应被复制
    }

    // 测试节点相等性
    #[test]
    fn test_equals() {
        let node1 = OcTreeDataNode::with_value(7);
        let node2 = OcTreeDataNode::with_value(7);
        assert!(node1.equals(&node2)); // 值相同，应相等
    }

    // 测试子节点存在性
    #[test]
    fn test_child_exists() {
        let mut node = OcTreeDataNode::<i32>::new();
        node.alloc_children(); // 分配子节点
        assert!(!node.child_exists(0)); // 子节点应为None
    }

    // 测试是否有子节点
    #[test]
    fn test_has_children() {
        let mut node = OcTreeDataNode::<i32>::new();
        assert!(!node.has_children()); // 初始应无子节点
        node.alloc_children(); // 分配子节点
        assert!(!node.has_children()); // 子节点仍为None
    }

    // 测试读写数据
    #[test]
    fn test_read_write_data() {
        let mut node = OcTreeDataNode::with_value(123);
        let mut buffer = Vec::new();
        node.write_data(&mut buffer).unwrap(); // 写入数据
        let mut new_node = OcTreeDataNode::<i32>::new();
        new_node.read_data(&mut buffer.as_slice()).unwrap(); // 读取数据
        assert_eq!(new_node.value, 123); // 值应相同
    }
}