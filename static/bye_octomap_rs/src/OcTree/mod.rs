// 这是一个在八叉树（Octree）结构中用于存储和管理占用信息的抽象基类。
pub mod AbstractOccupancyOcTree;

// 这是一个在八叉树（Octree）结构中用于存储和管理节点信息的抽象基类。
pub mod AbstractOcTree;

// 八叉树着色器
pub mod ColorOcTree;

// 八叉树计数器
pub mod CountingOcTree;

// MCTables数据表
pub mod MCTables;

/* start 八叉树数据结构相关 */

// 八叉树数据结构总成
pub mod OcTree;

// 八叉树节点, 重新导出struct
pub mod OcTreeNode;
pub use OcTreeNode::OcTreeNode as Node;

// 八叉树标记
pub mod OcTreeStamped;

// 八叉树迭代器
pub mod OcTreeIterator;

// 八叉树键值, 重新导出struct
pub mod OcTreeKey;
pub use OcTreeKey::OcTreeKey as Key;
pub use OcTreeKey::OcTreeKeyRay as KeyRay;
pub use OcTreeKey::OcTreeKeySet as KeySet;
pub use OcTreeKey::OcTreeKeyType as KeyType;

// 八叉树占据图的基类
pub mod OccupancyOcTreeBase;

// 八叉树基类
pub mod OcTreeBase;
pub mod OcTreeBaseImpl;

// 八叉树数据节点, 重新导出struct
pub mod OcTreeDataNode;
pub use OcTreeDataNode::OcTreeDataNode as DataNode;

/* end 八叉树数据结构相关 */
