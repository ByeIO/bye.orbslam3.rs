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

/* start ScanNode部分 */

// ScanNode 结构体
#[derive(Debug)]
pub struct ScanNode {
    pub scan: Option<Pointcloud>, // 点云数据
    pub pose: Pose6d,             // 6D姿态
    pub id: u32,                  // 节点ID
}

impl ScanNode {
    // 构造函数
    pub fn new(scan: Option<Pointcloud>, pose: Pose6d, id: u32) -> Self {
        ScanNode { scan, pose, id }
    }

    // 默认构造函数
    pub fn default() -> Self {
        ScanNode {
            scan: None,
            pose: Pose6D::new(),
            id: 0,
        }
    }

    // 使用base64编码进行二进制写入
    pub fn write_binary(&self, s: &mut impl Write) -> io::Result<()> {
        let mut binary_data = Vec::new();

        // 将Pointcloud数据写入二进制缓冲区
        if let Some(scan) = &self.scan {
            let point_count = scan.points.len() as u64;
            binary_data.extend_from_slice(&point_count.to_le_bytes());
            for point in &scan.points {
                binary_data.extend_from_slice(&point.x().to_le_bytes());
                binary_data.extend_from_slice(&point.y().to_le_bytes());
                binary_data.extend_from_slice(&point.z().to_le_bytes());
            }
        }

        // 将Pose6D数据写入二进制缓冲区
        let trans = self.pose.trans();
        binary_data.extend_from_slice(&trans.x().to_le_bytes());
        binary_data.extend_from_slice(&trans.y().to_le_bytes());
        binary_data.extend_from_slice(&trans.z().to_le_bytes());
        let rot = self.pose.rot();
        binary_data.extend_from_slice(&rot.u().to_le_bytes());
        binary_data.extend_from_slice(&rot.x().to_le_bytes());
        binary_data.extend_from_slice(&rot.y().to_le_bytes());
        binary_data.extend_from_slice(&rot.z().to_le_bytes());
        binary_data.extend_from_slice(&self.id.to_le_bytes());

        // 将二进制数据编码为base64并写入输出流
        let encoded = general_purpose::STANDARD.encode(&binary_data);
        s.write_all(encoded.as_bytes())?;
        Ok(())
    }

    // 使用base64解码进行二进制读取
    pub fn read_binary(&mut self, s: &mut impl Read) -> io::Result<()> {
        let mut encoded_data = Vec::new();
        s.read_to_end(&mut encoded_data)?;

        // 将base64数据解码为二进制
        let decoded_data = general_purpose::STANDARD.decode(&encoded_data)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        let mut cursor = io::Cursor::new(decoded_data);

        // 读取Pointcloud数据
        let mut point_count = 0;
        let mut buf = [0u8; 8];
        if cursor.read_exact(&mut buf).is_ok() {
            point_count = u64::from_le_bytes(buf) as usize;
        }
        self.scan = Some(Pointcloud::new());
        if let Some(scan) = &mut self.scan {
            for _ in 0..point_count {
                cursor.read_exact(&mut buf)?;
                let x = f64::from_le_bytes(buf);
                cursor.read_exact(&mut buf)?;
                let y = f64::from_le_bytes(buf);
                cursor.read_exact(&mut buf)?;
                let z = f64::from_le_bytes(buf);
                scan.push_back(x, y, z);
            }
        }

        // 读取Pose6D数据
        let mut buf = [0u8; 8];
        cursor.read_exact(&mut buf)?;
        let x = f64::from_le_bytes(buf);
        cursor.read_exact(&mut buf)?;
        let y = f64::from_le_bytes(buf);
        cursor.read_exact(&mut buf)?;
        let z = f64::from_le_bytes(buf);
        let trans = Vector3::from_xyz(x, y, z);
        cursor.read_exact(&mut buf)?;
        let u = f64::from_le_bytes(buf);
        cursor.read_exact(&mut buf)?;
        let x = f64::from_le_bytes(buf);
        cursor.read_exact(&mut buf)?;
        let y = f64::from_le_bytes(buf);
        cursor.read_exact(&mut buf)?;
        let z = f64::from_le_bytes(buf);
        let rot = Quaternion::from_uxyz(u, x, y, z);
        self.pose = Pose6D::from_trans_rot(trans, rot);

        // 读取ID
        let mut id_bytes = [0u8; 4];
        cursor.read_exact(&mut id_bytes)?;
        self.id = u32::from_le_bytes(id_bytes);

        Ok(())
    }

    // ASCII 写入姿态, 写入时固定精度
    pub fn write_pose_ascii(&self, s: &mut impl Write) -> io::Result<()> {
        write!(s, "{} ", self.id)?;
        let trans = self.pose.trans();
        write!(s, "{} {} {} ", trans.x(), trans.y(), trans.z())?;
        let euler = self.pose.rot().to_euler();
        write!(s, "{:.15} {:.15} {:.15}\n", euler.x(), euler.y(), euler.z())?;
        Ok(())
    }

    // ASCII 读取姿态
    pub fn read_pose_ascii(&mut self, s: &mut impl Read) -> io::Result<()> {
        let mut buffer = String::new();
        s.read_to_string(&mut buffer)?;
        let parts: Vec<&str> = buffer.split_whitespace().collect();
        if parts.len() < 7 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "数据格式错误"));
        }
        let id = parts[0].parse::<u32>().map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "ID解析失败"))?;
        if id != self.id {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "ID不匹配"));
        }
        let x = parts[1].parse::<f64>().map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "x解析失败"))?;
        let y = parts[2].parse::<f64>().map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "y解析失败"))?;
        let z = parts[3].parse::<f64>().map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "z解析失败"))?;
        let roll = parts[4].parse::<f64>().map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "roll解析失败"))?;
        let pitch = parts[5].parse::<f64>().map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "pitch解析失败"))?;
        let yaw = parts[6].parse::<f64>().map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "yaw解析失败"))?;
        self.pose = Pose6D::from_xyz_rpy(x, y, z, roll, pitch, yaw);
        Ok(())
    }

}

// 比较trait的实现
impl PartialEq for ScanNode {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

// 克隆trait的实现
impl Clone for ScanNode {
    fn clone(&self) -> Self {
        ScanNode {
            scan: self.scan.clone(),
            pose: self.pose.clone(),
            id: self.id,
        }
    }
}

#[cfg(test)]
mod tests1 {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_scan_node_new() {
        // 测试构造函数
        let pointcloud = Pointcloud::new();
        let pose = Pose6D::new();
        let scan_node = ScanNode::new(Some(pointcloud), pose, 1);

        assert!(scan_node.scan.is_some()); // 检查点云数据是否被正确设置
        assert_eq!(scan_node.id, 1); // 检查ID是否正确
    }

    #[test]
    fn test_scan_node_default() {
        // 测试默认构造函数
        let scan_node = ScanNode::default();

        assert!(scan_node.scan.is_none()); // 检查点云数据是否为空
        assert_eq!(scan_node.id, 0); // 检查ID是否为默认值
    }

    #[test]
    fn test_scan_node_write_read_binary() {
        // 测试二进制写入和读取
        let mut pointcloud = Pointcloud::new();
        pointcloud.push_back(1.0, 2.0, 3.0);
        let pose = Pose6D::from_xyz_rpy(1.0, 2.0, 3.0, 0.1, 0.2, 0.3);
        let scan_node = ScanNode::new(Some(pointcloud), pose, 1);

        // 写入到文件
        let mut file = File::create("./target/scan_node.bin.base64").unwrap();
        scan_node.write_binary(&mut file).unwrap(); // 写入二进制数据

        // 从文件读取
        let mut new_scan_node = ScanNode::default();
        let mut file = File::open("./target/scan_node.bin.base64").unwrap();
        new_scan_node.read_binary(&mut file).unwrap(); // 读取二进制数据

        assert_eq!(scan_node, new_scan_node); // 检查读取后的数据是否与写入前一致
    }

    #[test]
    fn test_scan_node_write_read_pose_ascii() {
        let pose = Pose6D::from_xyz_rpy(1.0, 2.0, 3.0, 0.1, 0.2, 0.3);
        let scan_node = ScanNode::new(None, pose, 1);
    
        // 写入到文件
        let mut file = File::create("./target/scan_node.ascii").unwrap();
        scan_node.write_pose_ascii(&mut file).unwrap();
    
        // 从文件读取
        let mut new_scan_node = ScanNode::default();
        new_scan_node.id = 1; // 设置ID
        let mut file = File::open("./target/scan_node.ascii").unwrap();
        new_scan_node.read_pose_ascii(&mut file).unwrap();
        
        assert_eq!(scan_node.pose, new_scan_node.pose);
    }

    #[test]
    fn test_scan_node_clone() {
        // 测试克隆功能
        let pointcloud = Pointcloud::new();
        let pose = Pose6D::new();
        let scan_node = ScanNode::new(Some(pointcloud), pose, 1);

        let cloned_node = scan_node.clone(); // 克隆节点

        assert_eq!(scan_node, cloned_node); // 检查克隆后的节点是否与原节点一致
    }

    #[test]
    fn test_scan_node_partial_eq() {
        // 测试PartialEq trait的实现
        let scan_node1 = ScanNode::new(None, Pose6D::new(), 1);
        let scan_node2 = ScanNode::new(None, Pose6D::new(), 1);

        assert_eq!(scan_node1, scan_node2); // 检查两个节点是否相等
    }
}

/* end ScanNode部分 */

/* start ScanEdge部分 */
// ScanEdge 结构体，表示两个 ScanNode 之间的连接
#[derive(Debug)]
pub struct ScanEdge {
    pub first: Rc<RefCell<ScanNode>>,  // 第一个节点
    pub second: Rc<RefCell<ScanNode>>, // 第二个节点
    pub constraint: Pose6d,            // 两个节点之间的6D变换约束
    pub weight: f64,                    // 边的权重
}

impl ScanEdge {
    // 构造函数
    pub fn new(first: Rc<RefCell<ScanNode>>, second: Rc<RefCell<ScanNode>>, constraint: Pose6d) -> Self {
        ScanEdge {
            first,
            second,
            constraint,
            weight: 1.0,
        }
    }

    // 默认构造函数
    pub fn default() -> Self {
        ScanEdge {
            first: Rc::new(RefCell::new(ScanNode::default())),
            second: Rc::new(RefCell::new(ScanNode::default())),
            constraint: Pose6D::new(),
            weight: 1.0,
        }
    }

    // 使用base64编码进行二进制写入
    pub fn write_binary(&self, s: &mut impl Write) -> io::Result<()> {
        let mut binary_data = Vec::new();

        // 写入第一个节点的ID
        binary_data.extend_from_slice(&self.first.borrow().id.to_le_bytes());
        // 写入第二个节点的ID
        binary_data.extend_from_slice(&self.second.borrow().id.to_le_bytes());
        // 写入约束
        let trans = self.constraint.trans();
        binary_data.extend_from_slice(&trans.x().to_le_bytes());
        binary_data.extend_from_slice(&trans.y().to_le_bytes());
        binary_data.extend_from_slice(&trans.z().to_le_bytes());
        let rot = self.constraint.rot();
        binary_data.extend_from_slice(&rot.u().to_le_bytes());
        binary_data.extend_from_slice(&rot.x().to_le_bytes());
        binary_data.extend_from_slice(&rot.y().to_le_bytes());
        binary_data.extend_from_slice(&rot.z().to_le_bytes());
        // 写入权重
        binary_data.extend_from_slice(&self.weight.to_le_bytes());

        // 将二进制数据编码为base64并写入输出流
        let encoded = general_purpose::STANDARD.encode(&binary_data);
        s.write_all(encoded.as_bytes())?;
        Ok(())
    }

    // 使用base64解码进行二进制读取
    pub fn read_binary(&mut self, s: &mut impl Read, graph: &ScanGraph) -> io::Result<()> {
        let mut encoded_data = Vec::new();
        s.read_to_end(&mut encoded_data)?;

        // 将base64数据解码为二进制
        let decoded_data = general_purpose::STANDARD.decode(&encoded_data)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        let mut cursor = io::Cursor::new(decoded_data);

        // 读取第一个节点的ID
        let mut first_id = [0u8; 4];
        cursor.read_exact(&mut first_id)?;
        let first_id = u32::from_le_bytes(first_id);

        // 读取第二个节点的ID
        let mut second_id = [0u8; 4];
        cursor.read_exact(&mut second_id)?;
        let second_id = u32::from_le_bytes(second_id);

        // 从图中获取节点
        self.first = graph.get_node_by_id(first_id).ok_or(io::Error::new(io::ErrorKind::InvalidData, "第一个节点未找到"))?;
        self.second = graph.get_node_by_id(second_id).ok_or(io::Error::new(io::ErrorKind::InvalidData, "第二个节点未找到"))?;

        // 读取约束
        let mut buf = [0u8; 8];
        cursor.read_exact(&mut buf)?;
        let x = f64::from_le_bytes(buf);
        cursor.read_exact(&mut buf)?;
        let y = f64::from_le_bytes(buf);
        cursor.read_exact(&mut buf)?;
        let z = f64::from_le_bytes(buf);
        let trans = Vector3::from_xyz(x, y, z);
        cursor.read_exact(&mut buf)?;
        let u = f64::from_le_bytes(buf);
        cursor.read_exact(&mut buf)?;
        let x = f64::from_le_bytes(buf);
        cursor.read_exact(&mut buf)?;
        let y = f64::from_le_bytes(buf);
        cursor.read_exact(&mut buf)?;
        let z = f64::from_le_bytes(buf);
        let rot = Quaternion::from_uxyz(u, x, y, z);
        self.constraint = Pose6D::from_trans_rot(trans, rot);

        // 读取权重
        cursor.read_exact(&mut buf)?;
        self.weight = f64::from_le_bytes(buf);

        Ok(())
    }
}

/// 克隆特性
impl Clone for ScanEdge {
    fn clone(&self) -> Self {
        ScanEdge {
            first: Rc::clone(&self.first),
            second: Rc::clone(&self.second),
            constraint: self.constraint.clone(),
            weight: self.weight,
        }
    }
}

/// 比较特性
impl PartialEq for ScanEdge {
    fn eq(&self, other: &Self) -> bool {
        self.first.borrow().id == other.first.borrow().id &&
        self.second.borrow().id == other.second.borrow().id
    }
}

#[cfg(test)]
mod tests2 {
    use super::*;
    use std::rc::Rc;
    use std::cell::RefCell;

    // 测试 ScanEdge 的构造函数
    #[test]
    fn test_scan_edge_new() {
        let node1 = Rc::new(RefCell::new(ScanNode::default()));
        let node2 = Rc::new(RefCell::new(ScanNode::default()));
        let constraint = Pose6d::new();
        let edge = ScanEdge::new(Rc::clone(&node1), Rc::clone(&node2), constraint);

        // 验证节点是否正确
        assert_eq!(edge.first.borrow().id, node1.borrow().id);
        assert_eq!(edge.second.borrow().id, node2.borrow().id);
        // 验证权重是否为默认值
        assert_eq!(edge.weight, 1.0);
    }

    // 测试 ScanEdge 的默认构造函数
    #[test]
    fn test_scan_edge_default() {
        let edge = ScanEdge::default();

        // 验证默认值是否正确
        assert_eq!(edge.first.borrow().id, 0);
        assert_eq!(edge.second.borrow().id, 0);
        assert_eq!(edge.weight, 1.0);
    }

    // 测试 ScanEdge 的二进制写入和读取
    #[test]
    fn test_scan_edge_write_read_binary() {
        let mut graph = ScanGraph::new();
        let node1 = Rc::new(RefCell::new(ScanNode::default()));
        let node2 = Rc::new(RefCell::new(ScanNode::default()));
        graph.add_node(None, Pose6d::new());
        graph.add_node(None, Pose6d::new());

        let constraint = Pose6d::new();
        let mut edge = ScanEdge::new(Rc::clone(&node1), Rc::clone(&node2), constraint);

        // 写入二进制数据
        let mut buffer = Vec::new();
        edge.write_binary(&mut buffer).unwrap();

        // 读取二进制数据
        let mut new_edge = ScanEdge::default();
        new_edge.read_binary(&mut &buffer[..], &graph).unwrap();

        // 验证数据是否正确
        assert_eq!(new_edge.first.borrow().id, node1.borrow().id);
        assert_eq!(new_edge.second.borrow().id, node2.borrow().id);
        assert_eq!(new_edge.weight, 1.0);
    }

    // 测试 ScanEdge 的克隆特性
    #[test]
    fn test_scan_edge_clone() {
        let node1 = Rc::new(RefCell::new(ScanNode::default()));
        let node2 = Rc::new(RefCell::new(ScanNode::default()));
        let constraint = Pose6d::new();
        let edge = ScanEdge::new(Rc::clone(&node1), Rc::clone(&node2), constraint);

        let cloned_edge = edge.clone();

        // 验证克隆后的数据是否一致
        assert_eq!(cloned_edge.first.borrow().id, node1.borrow().id);
        assert_eq!(cloned_edge.second.borrow().id, node2.borrow().id);
        assert_eq!(cloned_edge.weight, 1.0);
    }

    // 测试 ScanEdge 的比较特性
    #[test]
    fn test_scan_edge_partial_eq() {
        let node1 = Rc::new(RefCell::new(ScanNode::default()));
        let node2 = Rc::new(RefCell::new(ScanNode::default()));
        let constraint = Pose6d::new();
        let edge1 = ScanEdge::new(Rc::clone(&node1), Rc::clone(&node2), constraint);
        let edge2 = ScanEdge::new(Rc::clone(&node1), Rc::clone(&node2), constraint);

        // 验证两个边是否相等
        assert_eq!(edge1, edge2);
    }
}

/* end ScanEdge部分 */


/* start ScanGraph部分 */
// ScanGraph 结构体，表示由 ScanNode 和 ScanEdge 组成的图
#[derive(Debug)]
pub struct ScanGraph {
    pub nodes: Vec<Rc<RefCell<ScanNode>>>, // 节点列表
    pub edges: Vec<Rc<RefCell<ScanEdge>>>, // 边列表
}

impl ScanGraph {
    // 构造函数
    pub fn new() -> Self {
        ScanGraph {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    // 清空图，删除所有节点和边
    pub fn clear(&mut self) {
        self.nodes.clear();
        self.edges.clear();
    }

    // 添加节点
    pub fn add_node(&mut self, scan: Option<Pointcloud>, pose: Pose6d) -> Rc<RefCell<ScanNode>> {
        let node = Rc::new(RefCell::new(ScanNode::new(scan, pose, self.nodes.len() as u32)));
        self.nodes.push(node.clone());
        node
    }

    // 添加边
    pub fn add_edge(&mut self, first: Rc<RefCell<ScanNode>>, second: Rc<RefCell<ScanNode>>, constraint: Pose6d) -> Rc<RefCell<ScanEdge>> {
        let edge = Rc::new(RefCell::new(ScanEdge::new(first, second, constraint)));
        self.edges.push(edge.clone());
        edge
    }

    // 连接前一个节点
    pub fn connect_previous(&mut self) {
        if self.nodes.len() >= 2 {
            let first = self.nodes[self.nodes.len() - 2].clone();
            let second = self.nodes[self.nodes.len() - 1].clone();
            let c = first.borrow().pose.inv() * second.borrow().pose;
            self.add_edge(first, second, c);
        }
    }

    // 导出某个点
    pub fn export_dot(&self, filename: &str) -> io::Result<()> {
        let mut outfile = File::create(filename)?;
        writeln!(outfile, "graph ScanGraph")?;
        writeln!(outfile, "{{")?;
        for edge in &self.edges {
            let edge = edge.borrow();
            writeln!(
                outfile,
                "{} -- {} [label={:.2}]",
                edge.first.borrow().id,
                edge.second.borrow().id,
                edge.constraint.trans_length()
            )?;
        }
        writeln!(outfile, "}}")?;
        Ok(())
    }

    // 根据ID获取节点
    pub fn get_node_by_id(&self, id: u32) -> Option<Rc<RefCell<ScanNode>>> {
        self.nodes.iter().find(|node| node.borrow().id == id).cloned()
    }

    // 检查边是否存在
    pub fn edge_exists(&self, first_id: u32, second_id: u32) -> bool {
        self.edges.iter().any(|edge| {
            let edge = edge.borrow();
            (edge.first.borrow().id == first_id && edge.second.borrow().id == second_id) ||
            (edge.first.borrow().id == second_id && edge.second.borrow().id == first_id)
        })
    }

    // 获取邻居节点的ID
    pub fn get_neighbor_ids(&self, id: u32) -> Vec<u32> {
        let mut res = Vec::new();
        if let Some(node) = self.get_node_by_id(id) {
            for other_node in &self.nodes {
                if other_node.borrow().id == id {
                    continue;
                }
                if self.edge_exists(id, other_node.borrow().id) {
                    res.push(other_node.borrow().id);
                }
            }
        }
        res
    }

    // 获取节点的出边
    pub fn get_out_edges(&self, node: Rc<RefCell<ScanNode>>) -> Vec<Rc<RefCell<ScanEdge>>> {
        self.edges.iter()
            .filter(|edge| edge.borrow().first.borrow().id == node.borrow().id)
            .cloned()
            .collect()
    }

    // 获取节点的入边
    pub fn get_in_edges(&self, node: Rc<RefCell<ScanNode>>) -> Vec<Rc<RefCell<ScanEdge>>> {
        self.edges.iter()
            .filter(|edge| edge.borrow().second.borrow().id == node.borrow().id)
            .cloned()
            .collect()
    }

    // 对每个扫描进行变换
    pub fn transform_scans(&self) {
        for node in &self.nodes {
            let mut node = node.borrow_mut();
            if let Some(scan) = &mut node.scan {
                let pose = node.pose.clone();
                if let Some(scan) = &mut node.scan {
                    scan.transform_absolute(pose);
                }
            }
        }
    }// end fn transform_scans

    // 裁剪图
    pub fn crop(&self, lower_bound: Point3d, upper_bound: Point3d) {
        for node in &self.nodes {
            let mut node = node.borrow_mut();
            if let Some(scan) = &mut node.scan {
                scan.crop(lower_bound.clone(), upper_bound.clone());
            }
        }
    }

    // 获取节点的数量
    pub fn get_num_points(&self, max_id: u32) -> usize {
        let mut retval = 0;
        
        for node in &self.nodes {
            if let Some(scan) = &node.borrow().scan {
                retval += scan.points.len();
            }
            if max_id > 0 && node.borrow().id == max_id {
                break;
            }
        }
        retval
    }

    // 将图保存为二进制文件
    pub fn write_binary(&self, filename: &str) -> io::Result<()> {
        let file = File::create(filename)?;
        let mut writer = BufWriter::new(file);
        self.write_binary_internal(&mut writer)
    }

    // 内部二进制写入逻辑
    fn write_binary_internal(&self, writer: &mut impl Write) -> io::Result<()> {
        // 写入节点数量
        let node_count = self.nodes.len() as u32;
        writer.write_all(&node_count.to_le_bytes())?;

        // 写入每个节点
        for node in &self.nodes {
            node.borrow().write_binary(writer)?;
        }

        // 写入边数量
        let edge_count = self.edges.len() as u32;
        writer.write_all(&edge_count.to_le_bytes())?;

        // 写入每条边
        for edge in &self.edges {
            edge.borrow().write_binary(writer)?;
        }

        Ok(())
    }

    // 从二进制文件读取图
    pub fn read_binary(&mut self, filename: &str) -> io::Result<()> {
        let file = File::open(filename)?;
        let mut reader = BufReader::new(file);
        self.read_binary_internal(&mut reader)
    }

    // 内部二进制读取逻辑
    fn read_binary_internal(&mut self, reader: &mut impl Read) -> io::Result<()> {
        self.clear();

        // 读取节点数量
        let mut node_count_buf = [0u8; 4];
        reader.read_exact(&mut node_count_buf)?;
        let node_count = u32::from_le_bytes(node_count_buf);

        // 读取每个节点
        for _ in 0..node_count {
            let mut node = ScanNode::default();
            node.read_binary(reader)?;
            self.nodes.push(Rc::new(RefCell::new(node)));
        }

        // 读取边数量
        let mut edge_count_buf = [0u8; 4];
        reader.read_exact(&mut edge_count_buf)?;
        let edge_count = u32::from_le_bytes(edge_count_buf);

        // 读取每条边
        for _ in 0..edge_count {
            let mut edge = ScanEdge::default();
            edge.read_binary(reader, self)?;
            self.edges.push(Rc::new(RefCell::new(edge)));
        }

        Ok(())
    }

    // 将图保存为ASCII文件
    pub fn write_ascii(&self, filename: &str) -> io::Result<()> {
        let file = File::create(filename)?;
        let mut writer = BufWriter::new(file);
        self.write_ascii_internal(&mut writer)
    }

    // 内部ASCII写入逻辑
    fn write_ascii_internal(&self, writer: &mut impl Write) -> io::Result<()> {
        // 写入节点姿态
        for node in &self.nodes {
            node.borrow().write_pose_ascii(writer)?;
        }

        // 写入边信息
        for edge in &self.edges {
            let edge = edge.borrow();
            writeln!(
                writer,
                "{} {} {} {} {} {} {} {} {}",
                edge.first.borrow().id,
                edge.second.borrow().id,
                edge.constraint.trans().x(),
                edge.constraint.trans().y(),
                edge.constraint.trans().z(),
                edge.constraint.rot().u(),
                edge.constraint.rot().x(),
                edge.constraint.rot().y(),
                edge.constraint.rot().z(),
            )?;
        }

        Ok(())
    }

    // 从ASCII文件读取图
    pub fn read_ascii(&mut self, filename: &str) -> io::Result<()> {
        let file = File::open(filename)?;
        let mut reader = BufReader::new(file);
        self.read_ascii_internal(&mut reader)
    }

    // 内部ASCII读取逻辑
    fn read_ascii_internal(&mut self, reader: &mut impl Read) -> io::Result<()> {
        self.clear();

        let mut buffer = String::new();
        reader.read_to_string(&mut buffer)?;

        // 读取节点
        for line in buffer.lines() {
            if line.starts_with("NODE") {
                let mut node = ScanNode::default();
                node.read_pose_ascii(&mut line.as_bytes())?;
                self.nodes.push(Rc::new(RefCell::new(node)));
            }
        }

        // 读取边
        for line in buffer.lines() {
            if !line.starts_with("NODE") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() == 9 {

                    let first_id = parts[0].parse::<u32>().map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
                    let second_id = parts[1].parse::<u32>().map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
                    let trans = Vector3::from_xyz(
                        parts[2].parse::<f64>().map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?,
                        parts[3].parse::<f64>().map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?,
                        parts[4].parse::<f64>().map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?,
                    );
                    let rot = Quaternion::from_uxyz(
                        parts[5].parse::<f64>().map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?,
                        parts[6].parse::<f64>().map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?,
                        parts[7].parse::<f64>().map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?,
                        parts[8].parse::<f64>().map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?,
                    );
            
                    let constraint = Pose6D::from_trans_rot(trans, rot);
                    let first = self.get_node_by_id(first_id).unwrap();
                    let second = self.get_node_by_id(second_id).unwrap();
                    self.add_edge(first, second, constraint);
                }
            }
        }

        Ok(())
    }// end fn read_ascii_internal

}

/// 克隆特性
impl Clone for ScanGraph {
    fn clone(&self) -> Self {
        let mut nodes = Vec::new();
        let mut edges = Vec::new();

        // 克隆所有节点
        for node in &self.nodes {
            nodes.push(Rc::new(RefCell::new(node.borrow().clone())));
        }

        // 克隆所有边
        for edge in &self.edges {
            let first = self.get_node_by_id(edge.borrow().first.borrow().id).unwrap();
            let second = self.get_node_by_id(edge.borrow().second.borrow().id).unwrap();
            edges.push(Rc::new(RefCell::new(ScanEdge::new(
                first,
                second,
                edge.borrow().constraint.clone(),
            ))));
        }

        ScanGraph { nodes, edges }
    }
}

/// 比较特性
impl PartialEq for ScanGraph {
    fn eq(&self, other: &Self) -> bool {
        // 比较节点数量
        if self.nodes.len() != other.nodes.len() {
            return false;
        }

        // 比较边数量
        if self.edges.len() != other.edges.len() {
            return false;
        }

        // 比较每个节点的ID和姿态
        for (node1, node2) in self.nodes.iter().zip(other.nodes.iter()) {
            if node1.borrow().id != node2.borrow().id || node1.borrow().pose != node2.borrow().pose {
                return false;
            }
        }

        // 比较每条边的连接关系
        for (edge1, edge2) in self.edges.iter().zip(other.edges.iter()) {
            if edge1.borrow().first.borrow().id != edge2.borrow().first.borrow().id
                || edge1.borrow().second.borrow().id != edge2.borrow().second.borrow().id
            {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests3 {
    use super::*;
    use std::rc::Rc;
    use std::cell::RefCell;

    // 测试 ScanGraph 的构造函数
    #[test]
    fn test_scan_graph_new() {
        let graph = ScanGraph::new();

        // 验证初始状态
        assert_eq!(graph.nodes.len(), 0);
        assert_eq!(graph.edges.len(), 0);
    }

    // 测试添加节点
    #[test]
    fn test_scan_graph_add_node() {
        let mut graph = ScanGraph::new();
        let node = graph.add_node(None, Pose6d::new());

        // 验证节点是否被添加
        assert_eq!(graph.nodes.len(), 1);
        assert_eq!(node.borrow().id, 0);
    }

    // 测试添加边
    #[test]
    fn test_scan_graph_add_edge() {
        let mut graph = ScanGraph::new();
        let node1 = graph.add_node(None, Pose6d::new());
        let node2 = graph.add_node(None, Pose6d::new());
        let edge = graph.add_edge(Rc::clone(&node1), Rc::clone(&node2), Pose6d::new());

        // 验证边是否被添加
        assert_eq!(graph.edges.len(), 1);
        assert_eq!(edge.borrow().first.borrow().id, node1.borrow().id);
        assert_eq!(edge.borrow().second.borrow().id, node2.borrow().id);
    }

    // 测试根据ID获取节点
    #[test]
    fn test_scan_graph_get_node_by_id() {
        let mut graph = ScanGraph::new();
        let node = graph.add_node(None, Pose6d::new());

        // 验证节点是否可以通过ID获取
        let found_node = graph.get_node_by_id(0);
        assert!(found_node.is_some());
        assert_eq!(found_node.unwrap().borrow().id, node.borrow().id);
    }

    // 测试边是否存在
    #[test]
    fn test_scan_graph_edge_exists() {
        let mut graph = ScanGraph::new();
        let node1 = graph.add_node(None, Pose6d::new());
        let node2 = graph.add_node(None, Pose6d::new());
        graph.add_edge(Rc::clone(&node1), Rc::clone(&node2), Pose6d::new());

        // 验证边是否存在
        assert!(graph.edge_exists(0, 1));
        assert!(!graph.edge_exists(1, 2));
    }

    // 测试获取邻居节点的ID
    #[test]
    fn test_scan_graph_get_neighbor_ids() {
        let mut graph = ScanGraph::new();
        let node1 = graph.add_node(None, Pose6d::new());
        let node2 = graph.add_node(None, Pose6d::new());
        graph.add_edge(Rc::clone(&node1), Rc::clone(&node2), Pose6d::new());

        // 验证邻居节点ID是否正确
        let neighbors = graph.get_neighbor_ids(0);
        assert_eq!(neighbors.len(), 1);
        assert_eq!(neighbors[0], 1);
    }

    // 测试图的克隆
    #[test]
    fn test_scan_graph_clone() {
        let mut graph = ScanGraph::new();
        let node1 = graph.add_node(None, Pose6d::new());
        let node2 = graph.add_node(None, Pose6d::new());
        graph.add_edge(Rc::clone(&node1), Rc::clone(&node2), Pose6d::new());

        let cloned_graph = graph.clone();

        // 验证克隆后的图是否一致
        assert_eq!(cloned_graph.nodes.len(), graph.nodes.len());
        assert_eq!(cloned_graph.edges.len(), graph.edges.len());
    }

    // 测试图的比较
    #[test]
    fn test_scan_graph_partial_eq() {
        let mut graph1 = ScanGraph::new();
        let node1 = graph1.add_node(None, Pose6d::new());
        let node2 = graph1.add_node(None, Pose6d::new());
        graph1.add_edge(Rc::clone(&node1), Rc::clone(&node2), Pose6d::new());

        let mut graph2 = ScanGraph::new();
        let node1 = graph2.add_node(None, Pose6d::new());
        let node2 = graph2.add_node(None, Pose6d::new());
        graph2.add_edge(Rc::clone(&node1), Rc::clone(&node2), Pose6d::new());

        // 验证两个图是否相等
        assert_eq!(graph1, graph2);
    }
}

/* end ScanGraph部分 */