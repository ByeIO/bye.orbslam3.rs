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
use std::f64;
use base64::{engine::general_purpose, Engine as _};

use crate::Types::{Point3d, Pose6d, Point3dCollection, Point3dList, OcTreeVolume, Vector3, Quaternion, Pose6D};
use crate::PointCloud::Pointcloud;
use crate::Utils::{
    deg_to_rad, rad_to_deg, logodds, probability,
};
use crate::ScanGraph::{
    ScanNode, ScanEdge, ScanGraph,
};
use crate::OcTree::DataNode;

/// OcTreeNode 表示 3D 占用网格单元
/// "value" 存储其 log-odds 占用率
pub struct OcTreeNode {
    pub data_node: DataNode<f32>,
}

impl OcTreeNode {
    /// 创建一个新的 OcTreeNode
    pub fn new() -> Self {
        OcTreeNode {
            data_node: DataNode::with_value(0.0),
        }
    }

    // -- 节点占用率相关方法 ----------------------------

    /// 获取节点的占用概率
    pub fn get_occupancy(&self) -> f64 {
        probability(self.data_node.value.into())
    }

    /// 获取节点占用概率的 log odds 表示
    pub fn get_log_odds(&self) -> f32 {
        self.data_node.value
    }

    /// 设置节点的 log odds 占用率
    pub fn set_log_odds(&mut self, log_odds: f32) {
        self.data_node.value = log_odds;
    }

    /// 获取所有子节点占用概率的 log odds 平均值
    pub fn get_mean_child_log_odds(&self) -> f64 {
        let mut mean = 0.0;
        let mut count = 0;

        if let Some(children) = &self.data_node.children {
            for child in children {
                if let Some(child_node) = child {
                    // 直接访问 value 字段计算概率
                    mean += probability(child_node.value.into());
                    count += 1;
                }
            }
        }

        if count > 0 {
            mean /= count as f64;
            (mean / (1.0 - mean)).ln()
        } else {
            0.0
        }
    }

    /// 获取子节点占用概率的 log odds 最大值
    pub fn get_max_child_log_odds(&self) -> f32 {
        let mut max = f32::MIN;

        if let Some(children) = &self.data_node.children {
            for child in children {
                if let Some(child_node) = child {
                    // 直接比较 value 字段
                    if child_node.value > max {
                        max = child_node.value;
                    }
                }
            }
        }

        max
    }

    /// 根据子节点的最大占用率更新当前节点的占用率
    pub fn update_occupancy_children(&mut self) {
        self.set_log_odds(self.get_max_child_log_odds());
    }

    /// 将 p 添加到节点的 logOdds 值（不进行边界/阈值检查）
    pub fn add_value(&mut self, log_odds: f32) {
        self.data_node.value += log_odds;
    }
}

#[cfg(test)]
mod tests1 {
    use super::*;

    #[test]
    fn test_octree_node() {
        let mut node = OcTreeNode::new();
        assert_eq!(node.get_log_odds(), 0.0);

        node.set_log_odds(0.5);
        assert_eq!(node.get_log_odds(), 0.5);

        node.add_value(0.3);
        assert_eq!(node.get_log_odds(), 0.8);

        let occupancy = node.get_occupancy();
        assert!(occupancy > 0.0 && occupancy < 1.0);

        let mean_log_odds = node.get_mean_child_log_odds();
        assert_eq!(mean_log_odds, 0.0);

        let max_log_odds = node.get_max_child_log_odds();
        assert_eq!(max_log_odds, f32::MIN);
    }
}