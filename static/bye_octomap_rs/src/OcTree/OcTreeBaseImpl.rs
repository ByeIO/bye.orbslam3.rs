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

/// OcTreeBaseImpl 是 OcTree 的基类，用于与任何类型的 OcTreeDataNode 一起使用。
/// 该树实现目前的最大深度为 16 个节点。因此，坐标值必须低于 +/- 327.68 米（最大分辨率为 0.01 米）。
/// 此限制使得可以使用一种高效的键生成方法，该方法使用数据点坐标的二进制表示。
/// 注意：您可能不应直接使用此类，而是使用 OcTreeBase 或 OccupancyOcTreeBase。
/// 模板参数：
/// - NODE: 树中使用的节点类（通常派生自 OcTreeDataNode）
/// - INTERFACE: 要派生的接口，应为 AbstractOcTree 或 AbstractOccupancyOcTree
/// 
// ### 主要函数
// 1. **构造函数与析构函数**  
//    - `OcTreeBaseImpl(double resolution)`：初始化八叉树，设置分辨率。
//    - `~OcTreeBaseImpl()`：析构函数，释放内存。
//    - `OcTreeBaseImpl(const OcTreeBaseImpl<NODE,INTERFACE>& rhs)`：深拷贝构造函数。

// 2. **树结构操作**  
//    - `swapContent(OcTreeBaseImpl<NODE,INTERFACE>& rhs)`：交换两棵树的内容。
//    - `operator== (const OcTreeBaseImpl<NODE,INTERFACE>& rhs)`：比较两棵树是否相同。
//    - `setResolution(double r)`：设置树的分辨率。
//    - `getResolution()`：获取树的分辨率。
//    - `getTreeDepth()`：获取树的深度。
//    - `getNodeSize(unsigned depth)`：获取指定深度的节点大小。

// 3. **节点操作**  
//    - `createNodeChild(NODE* node, unsigned int childIdx)`：创建节点的子节点。
//    - `deleteNodeChild(NODE* node, unsigned int childIdx)`：删除节点的子节点。
//    - `getNodeChild(NODE* node, unsigned int childIdx)`：获取节点的子节点。
//    - `isNodeCollapsible(const NODE* node)`：判断节点是否可压缩。
//    - `nodeChildExists(const NODE* node, unsigned int childIdx)`：判断节点的子节点是否存在。
//    - `nodeHasChildren(const NODE* node)`：判断节点是否有子节点。
//    - `expandNode(NODE* node)`：扩展节点。
//    - `pruneNode(NODE* node)`：压缩节点。

// 4. **搜索与删除**  
//    - `search(double x, double y, double z, unsigned int depth)`：在指定深度搜索节点。
//    - `deleteNode(double x, double y, double z, unsigned int depth)`：删除指定位置的节点。

// 5. **树的操作**  
//    - `clear()`：清空树。
//    - `prune()`：压缩树。
//    - `expand()`：扩展树。

// 6. **统计与度量**  
//    - `size()`：获取树中节点的数量。
//    - `memoryUsage()`：获取树的内存使用情况。
//    - `memoryFullGrid()`：获取完整网格的内存使用情况。
//    - `volume()`：计算树的体积。
//    - `getMetricSize(double& x, double& y, double& z)`：获取树的尺寸。
//    - `getMetricMin(double& x, double& y, double& z)`：获取树的最小边界。
//    - `getMetricMax(double& x, double& y, double& z)`：获取树的最大边界。
//    - `calcNumNodes()`：计算树中节点的总数。
//    - `getNumLeafNodes()`：获取树中叶节点的数量。

// 7. **射线追踪**  
//    - `computeRayKeys(const point3d& origin, const point3d& end, KeyRay& ray)`：计算射线经过的节点。
//    - `computeRay(const point3d& origin, const point3d& end, std::vector<point3d>& ray)`：计算射线经过的节点坐标。

// 8. **文件IO**  
//    - `readData(std::istream &s)`：从输入流中读取树的数据。
//    - `writeData(std::ostream &s)`：将树的数据写入输出流。

// 9. **迭代器**  
//    - `begin()`：返回树的起始迭代器。
//    - `end()`：返回树的结束迭代器。
//    - `begin_leafs()`：返回叶节点的起始迭代器。
//    - `end_leafs()`：返回叶节点的结束迭代器。
//    - `begin_leafs_bbx()`：返回指定边界框内叶节点的起始迭代器。
//    - `end_leafs_bbx()`：返回指定边界框内叶节点的结束迭代器。
//    - `begin_tree()`：返回所有节点的起始迭代器。
//    - `end_tree()`：返回所有节点的结束迭代器。

// 10. **坐标与键转换**  
//     - `coordToKey(double coordinate)`：将坐标转换为键。
//     - `keyToCoord(key_type key)`：将键转换为坐标。
//     - `coordToKeyChecked(double coordinate, key_type& key)`：将坐标转换为键并进行边界检查。

// 11. **递归操作**  
//     - `deleteNodeRecurs(NODE* node)`：递归删除节点。
//     - `pruneRecurs(NODE* node, unsigned int depth, unsigned int max_depth, unsigned int& num_pruned)`：递归压缩节点。
//     - `expandRecurs(NODE* node, unsigned int depth, unsigned int max_depth)`：递归扩展节点。
//     - `calcNumNodesRecurs(NODE* node, size_t& num_nodes)`：递归计算节点数量。
//     - `getNumLeafNodesRecurs(const NODE* parent)`：递归计算叶节点数量。

// ### 其他函数
// - `init()`：初始化树的非平凡成员。
// - `calcMinMax()`：计算树的最小和最大边界。
// - `allocNodeChildren(NODE* node)`：为节点分配子节点。

use std::collections::LinkedList;
use std::fs::File;
use std::io::{self, BufWriter, Write, Read, BufReader};
use std::path::Path;
use std::rc::Rc;
use std::cell::{RefMut, Ref, RefCell};
use std::marker::PhantomData;

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
    DataNode, Node, Key, 
    KeyRay, KeySet, KeyType, 
};

/// NODE和INTERFACE为范型参数
pub struct OcTreeBaseImpl<NODE, INTERFACE> {
    // 树的根节点，空树时为 None
    pub root: Option<Box<NODE>>, 
    // 树的最大深度，目前固定为 16
    pub tree_depth: u8, 
    // 树的最大值
    pub tree_max_val: u32, 
    // 分辨率，单位为米
    pub resolution: f64, 
    // 分辨率的倒数，即 1.0 / resolution
    pub resolution_factor: f64, 
    // 树中的节点数量
    pub tree_size: usize, 
    // 标志，表示树的尺寸是否发生变化（用于延迟最小/最大评估）
    pub size_changed: bool, 
    // 树的中心坐标偏移
    pub tree_center: Point3d, 
    // x, y, z 的最大值
    pub max_value: [f64; 3], 
    // x, y, z 的最小值
    pub min_value: [f64; 3], 
    // 包含每个层级（0：根节点）的体素大小，tree_depth+1 个层级（包括 0）
    pub size_lookup_table: Vec<f64>,
    // 用于射线追踪的数据结构，数组用于多线程
    pub keyrays: Vec<KeyRay>, 
    // 使用 PhantomData 来标记未使用的 INTERFACE 类型参数
    _marker: PhantomData<INTERFACE>,
}

// 实现比较trait
impl<NODE, INTERFACE> PartialEq for OcTreeBaseImpl<NODE, INTERFACE>
where
    NODE: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.root == other.root
            && self.tree_depth == other.tree_depth
            && self.tree_max_val == other.tree_max_val
            && self.resolution == other.resolution
            && self.resolution_factor == other.resolution_factor
            && self.tree_size == other.tree_size
            && self.size_changed == other.size_changed
            && self.tree_center == other.tree_center
            && self.max_value == other.max_value
            && self.min_value == other.min_value
            && self.size_lookup_table == other.size_lookup_table
            && self.keyrays == other.keyrays
    }
}

impl<NODE, INTERFACE> Eq for OcTreeBaseImpl<NODE, INTERFACE> where NODE: Eq {}

// 实现克隆trait
impl<NODE, INTERFACE> Clone for OcTreeBaseImpl<NODE, INTERFACE>
where
    NODE: Clone,
{
    fn clone(&self) -> Self {
        Self {
            root: self.root.clone(),
            tree_depth: self.tree_depth,
            tree_max_val: self.tree_max_val,
            resolution: self.resolution,
            resolution_factor: self.resolution_factor,
            tree_size: self.tree_size,
            size_changed: self.size_changed,
            tree_center: self.tree_center.clone(),
            max_value: self.max_value,
            min_value: self.min_value,
            size_lookup_table: self.size_lookup_table.clone(),
            keyrays: self.keyrays.clone(),
            _marker: PhantomData,
        }
    }
}

impl<NODE, INTERFACE> OcTreeBaseImpl<NODE, INTERFACE> {
    /// 1. **构造函数与析构函数**  构造函数，初始化 OcTreeBaseImpl
    pub fn new(resolution: f64) -> Self {
        Self {
            root: None,
            tree_depth: 16,
            tree_max_val: 32768,
            resolution,
            resolution_factor: 1.0 / resolution,
            tree_size: 0,
            size_changed: true,
            tree_center: Point3d::new(),
            max_value: [0.0; 3],
            min_value: [0.0; 3],
            size_lookup_table: Vec::new(),
            keyrays: Vec::new(),
            _marker: PhantomData,
        }
    }

    // 2. **树结构操作**  

    /// 交换两棵树的内容
    pub fn swap_content(&mut self, other: &mut OcTreeBaseImpl<NODE, INTERFACE>) {
        // 交换根节点
        let this_root = self.root.take();
        self.root = other.root.take();
        other.root = this_root;

        // 交换树的大小
        let this_size = self.tree_size;
        self.tree_size = other.tree_size;
        other.tree_size = this_size;

        // 交换其他需要交换的字段
        std::mem::swap(&mut self.tree_depth, &mut other.tree_depth);
        std::mem::swap(&mut self.tree_max_val, &mut other.tree_max_val);
        std::mem::swap(&mut self.resolution, &mut other.resolution);
        std::mem::swap(&mut self.resolution_factor, &mut other.resolution_factor);
        std::mem::swap(&mut self.size_changed, &mut other.size_changed);
        std::mem::swap(&mut self.tree_center, &mut other.tree_center);
        std::mem::swap(&mut self.max_value, &mut other.max_value);
        std::mem::swap(&mut self.min_value, &mut other.min_value);
        std::mem::swap(&mut self.size_lookup_table, &mut other.size_lookup_table);
        std::mem::swap(&mut self.keyrays, &mut other.keyrays);
    }

    // 设置树的分辨率，缩放所有体素
    pub fn set_resolution(&mut self, resolution: f64) {
        self.resolution = resolution;
        self.resolution_factor = 1.0 / resolution;

        // 初始化节点大小查找表
        self.size_lookup_table.resize(self.tree_depth as usize + 1, 0.0);
        for i in 0..=self.tree_depth {
            self.size_lookup_table[i as usize] = self.resolution * (1 << (self.tree_depth - i)) as f64;
        }

        self.size_changed = true;
    }

    /// 获取树的分辨率
    pub fn get_resolution(&self) -> f64 {
        self.resolution
    }

    /// 获取树的深度
    pub fn get_tree_depth(&self) -> u8 {
        self.tree_depth
    }

    /// 获取指定深度的节点大小
    pub fn get_node_size(&self, depth: u8) -> f64 {
        assert!(depth <= self.tree_depth);
        self.size_lookup_table[depth as usize]
    }

    /// 清除 KeyRay 向量以最小化不必要的内存
    pub fn clear_key_rays(&mut self) {
        self.keyrays.clear();
    }

    // 3. **节点操作**  
    /* start 节点操作 */

    // /// 创建节点的子节点
    // pub fn create_node_child(&mut self, node: &mut NODE, child_idx: u8) -> &mut NODE {
    //     assert!(child_idx < 8);
    //     if node.children.is_none() {
    //         self.alloc_node_children(node);
    //     }
    //     assert!(node.children.as_ref().unwrap()[child_idx as usize].is_none());
    //     let new_node = Box::new(NODE::new());
    //     node.children.as_mut().unwrap()[child_idx as usize] = Some(new_node);

    //     self.tree_size += 1;
    //     self.size_changed = true;

    //     node.children.as_mut().unwrap()[child_idx as usize].as_mut().unwrap()
    // }

    // /// 删除节点的子节点
    // pub fn delete_node_child(&mut self, node: &mut NODE, child_idx: u8) {
    //     assert!(child_idx < 8 && node.children.is_some());
    //     assert!(node.children.as_ref().unwrap()[child_idx as usize].is_some());
    //     node.children.as_mut().unwrap()[child_idx as usize] = None;

    //     self.tree_size -= 1;
    //     self.size_changed = true;
    // }

    // /// 获取节点的子节点
    // pub fn get_node_child(&self, node: &NODE, child_idx: u8) -> &NODE {
    //     assert!(child_idx < 8 && node.children.is_some());
    //     assert!(node.children.as_ref().unwrap()[child_idx as usize].is_some());
    //     node.children.as_ref().unwrap()[child_idx as usize].as_ref().unwrap()
    // }

    // /// 获取节点的子节点（常量版本）
    // pub fn get_node_child_const(&self, node: &NODE, child_idx: u8) -> &NODE {
    //     assert!(child_idx < 8 && node.children.is_some());
    //     assert!(node.children.as_ref().unwrap()[child_idx as usize].is_some());
    //     node.children.as_ref().unwrap()[child_idx as usize].as_ref().unwrap()
    // }

    // /// 判断节点是否可折叠
    // pub fn is_node_collapsible(&self, node: &NODE) -> bool {
    //     // 所有子节点必须存在，且没有自己的子节点，并且具有相同的占用值
    //     if !self.node_child_exists(node, 0) {
    //         return false;
    //     }

    //     let first_child = self.get_node_child(node, 0);
    //     if self.node_has_children(first_child) {
    //         return false;
    //     }

    //     for i in 1..8 {
    //         if !self.node_child_exists(node, i) || self.node_has_children(self.get_node_child(node, i)) || !(self.get_node_child(node, i) == first_child) {
    //             return false;
    //         }
    //     }

    //     true
    // }

    // /// 判断节点是否具有子节点
    // pub fn node_child_exists(&self, node: &NODE, child_idx: u8) -> bool {
    //     assert!(child_idx < 8);
    //     if node.children.is_some() && node.children.as_ref().unwrap()[child_idx as usize].is_some() {
    //         true
    //     } else {
    //         false
    //     }
    // }

    // /// 判断节点是否具有任何子节点
    // pub fn node_has_children(&self, node: &NODE) -> bool {
    //     if node.children.is_none() {
    //         return false;
    //     }

    //     for i in 0..8 {
    //         if node.children.as_ref().unwrap()[i].is_some() {
    //             return true;
    //         }
    //     }
    //     false
    // }

    // /// 扩展节点（与剪枝相反）：创建所有子节点并将其占用概率设置为节点的值
    // pub fn expand_node(&mut self, node: &mut NODE) {
    //     assert!(!self.node_has_children(node));

    //     for k in 0..8 {
    //         let new_node = self.create_node_child(node, k);
    //         new_node.copy_data(node);
    //     }
    // }

    // /// 剪枝节点
    // pub fn prune_node(&mut self, node: &mut NODE) -> bool {
    //     if !self.is_node_collapsible(node) {
    //         return false;
    //     }

    //     // 将值设置为子节点的值（假设所有子节点值相同）
    //     node.copy_data(self.get_node_child(node, 0));

    //     // 删除子节点（已知此时为叶子节点）
    //     for i in 0..8 {
    //         self.delete_node_child(node, i);
    //     }
    //     node.children = None;

    //     true
    // }

    // /// 分配节点的子节点
    // pub fn alloc_node_children(&mut self, node: &mut NODE) {
    //     node.children = Some(vec![None; 8]);
    // }

    // 4. **搜索与删除**  
    // /// 搜索树中指定深度的节点
    // pub fn search(&self, x: f64, y: f64, z: f64, depth: u8) -> Option<&NODE> {
    //     let key = self.coord_to_key(x, y, z, depth);
    //     self.search_key(&key, depth)
    // }

    // /// 搜索树中指定深度的节点（使用 Point3d）
    // pub fn search_point(&self, point: &Point3d, depth: u8) -> Option<&NODE> {
    //     let key = self.coord_to_key(point.x(), point.y(), point.z(), depth);
    //     self.search_key(&key, depth)
    // }

    // /// 搜索树中指定深度的节点（使用 OcTreeKey）
    // pub fn search_key(&self, key: &OcTreeKey, depth: u8) -> Option<&NODE> {
    //     assert!(depth <= self.tree_depth);
    //     if self.root.is_none() {
    //         return None;
    //     }

    //     let mut current_node = self.root.as_ref().unwrap();
    //     let mut current_depth = 0;

    //     while current_depth < depth {
    //         let child_idx = self.compute_child_idx(key, self.tree_depth - 1 - current_depth);
    //         if self.node_child_exists(current_node, child_idx) {
    //             current_node = self.get_node_child(current_node, child_idx);
    //             current_depth += 1;
    //         } else {
    //             return None;
    //         }
    //     }

    //     Some(current_node)
    // }

    // /// 删除树中指定深度的节点
    // pub fn delete_node(&mut self, x: f64, y: f64, z: f64, depth: u8) -> bool {
    //     let key = self.coord_to_key(x, y, z, depth);
    //     self.delete_node_key(&key, depth)
    // }

    // /// 删除树中指定深度的节点（使用 OcTreeKey）
    // pub fn delete_node_key(&mut self, key: &OcTreeKey, depth: u8) -> bool {
    //     if self.root.is_none() {
    //         return true;
    //     }

    //     self.delete_node_recurs(self.root.as_mut().unwrap(), 0, depth, key)
    // }

    // /// 递归删除节点
    // fn delete_node_recurs(&mut self, node: &mut NODE, current_depth: u8, max_depth: u8, key: &OcTreeKey) -> bool {
    //     if current_depth >= max_depth {
    //         return true;
    //     }

    //     let child_idx = self.compute_child_idx(key, self.tree_depth - 1 - current_depth);
    //     if !self.node_child_exists(node, child_idx) {
    //         return false;
    //     }

    //     let delete_child = self.delete_node_recurs(
    //         self.get_node_child(node, child_idx),
    //         current_depth + 1,
    //         max_depth,
    //         key,
    //     );

    //     if delete_child {
    //         self.delete_node_child(node, child_idx);
    //         if !self.node_has_children(node) {
    //             return true;
    //         }
    //     }

    //     false
    // }

    // /// 递归删除节点及其子节点
    // fn delete_node_recurs(&mut self, node: Box<NODE>) {
    //     if let Some(children) = node.children.take() {
    //         for child in children.into_iter().flatten() {
    //             self.delete_node_recurs(child);
    //         }
    //     }
    // }

    // /* end 节点操作 */

    // /// 清除整个树结构
    // pub fn clear(&mut self) {
    //     if let Some(root) = self.root.take() {
    //         self.delete_node_recurs(root);
    //         self.tree_size = 0;
    //         self.size_changed = true;
    //     }
    // }

}