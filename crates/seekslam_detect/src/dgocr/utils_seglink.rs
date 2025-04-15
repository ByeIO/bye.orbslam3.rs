#![allow(unused)]

//! 检测(seglink技术)

// 线性代数库
use ndarray::{
    Array, ArrayD, ArrayViewD, 
    Dim, Ix1, Ix2, Ix3, Ix4, 
    IxDyn, Axis, IxDynImpl, 
    ArrayBase, OwnedRepr, IntoDimension,
    ArrayView, Array1, Array2, Array3,
    Array4, CowArray, ArrayView1, ShapeError,
};
use ndarray::s;
use ndarray::concatenate;
use ndarray::stack;

// 错误处理
use anyhow::{anyhow, Result};

// 图像处理相关模块
use image::{GenericImageView, ImageBuffer, Rgb, RgbImage, DynamicImage};

// 常量
const OFFSET_DIM: usize = 6;
const RBOX_DIM: usize = 5;
const N_LOCAL_LINKS: usize = 8;
const N_CROSS_LINKS:  usize = 4;
const N_SEG_CLASSES: usize = 2;
const N_LNK_CLASSES: usize = 4;
const MATCH_STATUS_POS: i32 = 1;
const MATCH_STATUS_NEG: i32 = -1;
const MATCH_STATUS_IGNORE: i32 = 0;
const MUT_LABEL: i32 = 3;
const POS_LABEL: i32 = 1;
const NEG_LABEL: i32 = 0;
const N_DET_LAYERS: usize = 6;
const FLAGS_NODE_THRESHOLD: f32 = 0.4;
const FLAGS_LINK_THRESHOLD: f32 = 0.6;

// 解码线段和链接
pub fn decode_segments_links_python(
    image_size: (usize, usize),
    all_nodes: Vec<Array2<f32>>,
    all_links: Vec<Array2<f32>>,
    all_reg: Vec<Array2<f32>>,
    anchor_sizes: Vec<f32>,
) -> Result<(Array2<f32>, Array1<i32>, Array1<i32>, Array1<i32>)> {
    // batch_size为1
    let batch_size = 1; 

    // 将输入的数据展平并拼接
    let all_nodes_flat = all_nodes
        .iter()
        .map(|o| o.view()
            .to_shape((batch_size, o.len() / (batch_size * N_SEG_CLASSES), N_SEG_CLASSES))
            .unwrap()
            .to_owned())
        // 直接收集为 Vec<Array3<f32>>
        .collect::<Vec<Array3<f32>>>() 
        .iter()
        .fold(Array3::zeros((batch_size, 0, N_SEG_CLASSES)), |acc, arr| {
            concatenate(Axis(1), &[acc.view(), arr.view()]).unwrap()
        });

    let all_links_flat = all_links
        .iter()
        .map(|o| o.view()
            .to_shape((batch_size, o.len() / (batch_size * N_LNK_CLASSES), N_LNK_CLASSES))
            .unwrap()
            .to_owned())
        // 直接收集为 Vec<Array3<f32>>
        .collect::<Vec<Array3<f32>>>() 
        .iter()
        .fold(Array3::zeros((batch_size, 0, N_LNK_CLASSES)), |acc, arr| {
            concatenate(Axis(1), &[acc.view(), arr.view()]).unwrap()
        });

    let all_reg_flat = all_reg
        .iter()
        .map(|o| o.view()
            .to_shape((batch_size, o.len() / (batch_size * OFFSET_DIM), OFFSET_DIM))
            .unwrap()
            .to_owned())
        // 直接收集为 Vec<Array3<f32>>
        .collect::<Vec<Array3<f32>>>()
        .iter()
        .fold(Array3::zeros((batch_size, 0, OFFSET_DIM)), |acc, arr| {
            concatenate(Axis(1), &[acc.view(), arr.view()]).unwrap()
        });

    // 使用decode_batch函数解码
    let (segments, group_indices, segment_counts, group_indices_all) =
        decode_batch(all_nodes_flat, all_links_flat, all_reg_flat, image_size, &anchor_sizes)?;

    Ok((segments, group_indices, segment_counts, group_indices_all))
}

// 定义一个函数，用于批量解码节点、链接和回归数据
pub fn decode_batch(
    // 输入参数：所有节点的得分，形状为 [batch_size, height, width]
    all_nodes: Array3<f32>,
    // 输入参数：所有链接的得分，形状为 [batch_size, height, width]
    all_links: Array3<f32>,
    // 输入参数：所有回归值，形状为 [batch_size, height, width]
    all_reg: Array3<f32>,
    // 输入参数：图像尺寸 (height, width)
    image_size: (usize, usize),
    // 输入参数：锚点尺寸的引用切片
    anchor_sizes: &[f32],
) -> Result<(Array2<f32>, Array1<i32>, Array1<i32>, Array1<i32>)> {
    // 获取批次大小
    let batch_size = all_nodes.shape()[0];
    // 初始化存储每个图像分割结果的向量
    let mut batch_segments = Vec::new();
    // 初始化存储每个图像分组索引的向量
    let mut batch_group_indices = Vec::new();
    // 初始化存储每个图像分割计数的向量
    let mut batch_segments_counts = Vec::new();
    // 初始化存储所有分组索引的向量
    let mut batch_group_indices_all = Vec::new();

    // 遍历批次中的每个图像
    for image_id in 0..batch_size {
        // 获取当前图像的节点得分切片
        let image_node_scores = all_nodes.slice(s![image_id, .., ..]);
        // 获取当前图像的链接得分切片
        let image_link_scores = all_links.slice(s![image_id, .., ..]);
        // 获取当前图像的回归值切片
        let image_reg = all_reg.slice(s![image_id, .., ..]);

        // 解码当前图像，获取分割结果、分组索引等
        let (image_segments, image_group_indices, image_segments_counts, image_group_indices_all) =
            decode_image(image_node_scores.to_owned(), image_link_scores.to_owned(), image_reg.to_owned(), image_size, anchor_sizes)?;

        // 将当前图像的结果存入对应的向量
        batch_segments.push(image_segments);
        batch_group_indices.push(image_group_indices);
        batch_segments_counts.push(image_segments_counts);
        batch_group_indices_all.push(image_group_indices_all);
    }

    // 找出所有图像中最大的分割计数
    let max_count = *batch_segments_counts.iter().max().unwrap();

    // 对每个图像进行填充，使其分割计数与最大计数一致
    for image_id in 0..batch_size {
        // 如果当前图像的分割计数不等于最大计数
        if batch_segments_counts[image_id] != max_count {
            // 计算需要填充的大小
            let pad_size = (max_count - batch_segments_counts[image_id]) as usize;
            // 创建用于填充的分割结果数组
            let batch_segments_pad = Array::from_shape_vec(
                (Dim::<[usize; 2]>::new([pad_size as usize, OFFSET_DIM])),
                vec![0.0; (pad_size * OFFSET_DIM).try_into().unwrap()],
            )
           .unwrap();
            // 将填充数组与原始数组拼接
            batch_segments[image_id] = ndarray::concatenate(
                Axis(0),
                &[batch_segments[image_id].view(), batch_segments_pad.view()]
            )?;

            // 创建用于填充的分组索引数组
            let batch_group_indices_pad = Array::from_shape_vec(
                (pad_size,),
                vec![-1; pad_size.try_into().unwrap()],
            )
           .unwrap();
            // 将填充数组与原始数组拼接
            batch_group_indices[image_id] = ndarray::concatenate(
                Axis(0),
                &[batch_group_indices[image_id].view(), batch_group_indices_pad.view()]
            )?;

        }
    }

    // 将所有图像的分割结果堆叠并转换为明确的所有权
    let a = ndarray::concatenate(Axis(0), &batch_segments.iter().map(|x| x.view()).collect::<Vec<ArrayView<f32, Dim<[usize; 2]> >>>()).to_owned()?; 
    // 将所有图像的分组索引堆叠成一个数组
    let b = ndarray::concatenate(Axis(0), &batch_group_indices.iter().map(|x| x.view()).collect::<Vec<ArrayView<i32, Dim<[usize; 1]> > > >())?;
    // 将所有图像的分割计数转换为数组
    let c = Array::from_iter(batch_segments_counts.iter().cloned());
    // 将所有图像的所有分组索引堆叠成一个数组
    let d = Array1::from_iter(batch_group_indices_all.iter().flat_map(|x| x.iter().cloned()));

    // 返回结果元组
    Ok((a, b, c, d))
}

// 定义函数decode_image，用于解码图像信息
pub fn decode_image(
    // 输入参数：图像节点得分矩阵，形状为二维数组
    image_node_scores: Array2<f32>,
    // 输入参数：图像链接得分矩阵，形状为二维数组
    image_link_scores: Array2<f32>,
    // 输入参数：图像回归参数矩阵，形状为二维数组
    image_reg: Array2<f32>,
    // 输入参数：图像尺寸，元组形式(宽度, 高度)
    image_size: (usize, usize),
    // 输入参数：锚点尺寸数组引用
    anchor_sizes: &[f32],
) -> Result<(Array2<f32>, Array1<i32>, i32, Array1<i32>)> {
    // 初始化存储各层特征图尺寸的向量
    let mut map_size = Vec::new();
    // 初始化存储各层默认偏移量的向量
    let mut offsets_defaults = Vec::new();
    // 初始化节点默认偏移量
    let mut offsets_default_node = 0;
    // 初始化链接默认偏移量
    let mut offsets_default_link = 0;

    // 遍历所有检测层
    for i in 0..N_DET_LAYERS {
        // 将当前层的节点和链接偏移量存入向量
        offsets_defaults.push((offsets_default_node, offsets_default_link));

        // 计算当前层的特征图尺寸（按比例缩小）
        let layer_size = (image_size.0 / (2_usize.pow((2 + i) as u32)), image_size.1 / (2_usize.pow((2 + i) as u32)));
        // 将当前层尺寸存入向量
        map_size.push(layer_size);
        // 更新节点偏移量（累加当前层节点数）
        offsets_default_node += layer_size.0 * layer_size.1;
        // 更新链接偏移量（根据是否为第一层决定链接数）
        if i == 0 {
            offsets_default_link += layer_size.0 * layer_size.1 * N_LOCAL_LINKS;
        } else {
            offsets_default_link += layer_size.0 * layer_size.1 * (N_LOCAL_LINKS + N_CROSS_LINKS);
        }
    }

    // 调用decode_image_by_join函数解码图像，获取分组索引
    let image_group_indices_all = decode_image_by_join(
        image_node_scores,
        image_link_scores,
        FLAGS_NODE_THRESHOLD,
        FLAGS_LINK_THRESHOLD,
        map_size.clone(),
        offsets_defaults.clone(),
    )?;

    // 将所有分组索引值减1（可能用于特殊标记）
    let image_group_indices_all = image_group_indices_all.mapv(|x| x - 1);
    // 筛选出不等于-1的分组索引（有效分组）
    // 生成掩码并直接用于过滤
    let image_group_indices: Vec<_> = image_group_indices_all
    .iter()
    // 使用闭包过滤有效值
    .filter(|&&x| x != -1)  
    // 解引用并克隆值（如果需要所有权）
    .cloned()  
    .collect();
    // 计算有效分组的数量
    let image_segments_counts = image_group_indices.len() as i32;

    // 初始化图像分割结果数组，形状为(分段数, 偏移维度)
    let mut image_segments = Array::zeros((image_segments_counts as usize, OFFSET_DIM));

    // 遍历所有有效分组索引
    for (i, offsets) in image_group_indices_all
       .iter()
       .enumerate()
       .filter(|&(_, &x)| x >= 0)
    {
        // 从回归矩阵中提取各个回归参数
        let encoded_cx = image_reg[(*offsets as usize, 0)];
        let encoded_cy = image_reg[(*offsets as usize, 1)];
        let encoded_width = image_reg[(*offsets as usize, 2)];
        let encoded_height = image_reg[(*offsets as usize, 3)];
        let encoded_theta_cos = image_reg[(*offsets as usize, 4)];
        let encoded_theta_sin = image_reg[(*offsets as usize, 5)];

        // 获取当前偏移量对应的坐标信息（层索引，x，y）
        let (l_idx, x, y) = get_coord(*offsets as usize, &map_size, &offsets_defaults)?;
        // 获取当前层的锚点尺寸
        let rs = anchor_sizes[l_idx];
        // 定义极小值常量，防止数值溢出
        let eps = 1e-6;

        // 计算并存储中心点x坐标（解码后的实际坐标）
        image_segments[(i, 0)] = encoded_cx * rs + (2_usize.pow((2 + l_idx) as u32) as f32) * (x as f32 + 0.5);
        // 计算并存储中心点y坐标（解码后的实际坐标）
        image_segments[(i, 1)] = encoded_cy * rs + (2_usize.pow((2 + l_idx) as u32) as f32) * (y as f32 + 0.5);
        // 计算并存储宽度（经过指数变换和解码）
        image_segments[(i, 2)] = (encoded_width.exp() * rs - eps) as f32;
        // 计算并存储高度（经过指数变换和解码）
        image_segments[(i, 3)] = (encoded_height.exp() * rs - eps) as f32;
        // 存储角度余弦值
        image_segments[(i, 4)] = encoded_theta_cos;
        // 存储角度正弦值
        image_segments[(i, 5)] = encoded_theta_sin;
    }

    // 返回解码结果：分割信息、有效分组索引、分段数量、全部分组索引
    Ok((image_segments, image_group_indices.into(), image_segments_counts, image_group_indices_all))
}

// 定义函数decode_image_by_join，用于通过连接算法解码图像
pub fn decode_image_by_join(
    // 节点得分矩阵，形状为[N, 2]，N是节点数量
    node_scores: Array2<f32>,
    // 连接得分矩阵，形状为[M, 2]，M是连接数量
    link_scores: Array2<f32>,
    // 节点得分阈值，高于此值被认为是有效节点
    node_threshold: f32,
    // 连接得分阈值，高于此值被认为是有效连接
    link_threshold: f32,
    // 各层的尺寸信息
    map_size: Vec<(usize, usize)>,
    // 各层的默认偏移量
    offsets_defaults: Vec<(usize, usize)>,
) -> Result<Array1<i32>> {

    // 创建节点掩码，只保留得分高于阈值的节点
    let node_mask = node_scores
       // 切片获取所有节点的正类得分
       .slice(s![.., POS_LABEL])
       // 将得分转换为布尔值，表示是否高于阈值
       .mapv(|x| x >= node_threshold)
       // 将布尔值转换为0或1
       .mapv(|x| if x { 1 } else { 0 })
       // 转换为动态维度数组
       .into_dyn();

    // 创建连接掩码，只保留得分高于阈值的连接
    let link_mask = link_scores
       // 切片获取所有连接的正类得分
       .slice(s![.., POS_LABEL])
       // 将得分转换为布尔值，表示是否高于阈值
       .mapv(|x| x >= link_threshold)
       // 将布尔值转换为0或1
       .mapv(|x| if x { 1 } else { 0 })
       // 转换为动态维度数组
       .into_dyn();

    // 初始化分组掩码，初始值都为-1
    let mut group_mask = Array::from_shape_vec(node_mask.shape().clone(), vec![-1; node_mask.len()]).unwrap().into_dyn();

    // 收集所有有效节点的位置索引
    let offsets_pos = node_mask
       // 遍历节点掩码
       .iter()
       .enumerate()
       // 过滤出有效节点
       .filter(|&(_, &x)| x == 1)
       // 只保留索引
       .map(|(i, _)| i)
       // 收集为Vec<usize>
       .collect::<Vec<usize>>();

    // 定义函数find_parent，查找某个点的父节点
    pub fn find_parent(group_mask: &mut Array1<i32>, point: usize) -> i32 {
        // 直接返回该点的父节点值
        group_mask[point]
    }

    // 定义函数set_parent，设置某个点的父节点
    pub fn set_parent(group_mask: &mut Array1<i32>, point: usize, parent: i32) {
        // 修改该点的父节点值
        group_mask[point] = parent;
    }

    // 定义函数is_root，判断某个点是否是根节点
    pub fn is_root(group_mask: &Array1<i32>, point: usize) -> bool {
        // 根节点的特征是父节点值为-1
        find_parent(&mut group_mask.clone(), point) == -1
    }

    // 定义函数find_root，查找某个点的根节点
    pub fn find_root(group_mask: &mut Array1<i32>, point: usize) -> usize {
        let mut root = point;
        let mut update_parent = false;
        // 循环查找直到找到根节点
        while!is_root(group_mask, root) {
            root = find_parent(group_mask, root) as usize;
            update_parent = true;
        }

        // 路径压缩优化：将查找路径上的节点直接指向根节点
        if update_parent {
            set_parent(group_mask, point, find_parent(&mut group_mask.clone(), root));
        }

        root
    }

    // 定义函数join，连接两个点
    pub fn join(group_mask: &mut Array1<i32>, p1: usize, p2: usize) {
        // 分别找到两个点的根节点
        let root1 = find_root(group_mask, p1);
        let root2 = find_root(group_mask, p2);

        // 如果根节点不同，则合并两个集合
        if root1 != root2 {
            set_parent(group_mask, root1, find_parent(&mut group_mask.clone(), root2));
        }
    }

    // 定义函数get_all，获取所有节点的分组结果
    pub fn get_all(group_mask: &Array1<i32>, offsets_pos: &[usize]) -> Array1<i32> {
        // 使用哈希表记录根节点到分组ID的映射
        let mut root_map = std::collections::HashMap::new();

        // 辅助函数，获取或创建根节点的分组ID
        pub fn get_index(root_map: &mut std::collections::HashMap<usize, usize>, root: usize) -> usize {
            let root_map_clone = root_map.clone();
            *root_map.entry(root).or_insert_with(|| root_map_clone.len() + 1)
        }

        // 初始化结果掩码
        let mut mask = Array::from_shape_vec(group_mask.shape().clone(), vec![0; group_mask.len()]).unwrap().into_dyn();
        // 遍历所有有效节点
        for (i, point) in offsets_pos.iter().enumerate() {
            // 查找当前节点的根节点
            let point_root = find_root(&mut group_mask.clone(), *point);
            // 获取或创建根节点的分组ID
            let bbox_idx = get_index(&mut root_map, point_root);
            // 在掩码中记录分组ID
            mask[*point] = bbox_idx as i32;
        }

        let fixed_mask = mask.into_dimensionality::<Ix1>().unwrap();
        // 返回值
        fixed_mask
    }

    // 根据连接关系合并节点
    let mut pos_link = 0;
    // 遍历所有有效节点
    for (i, offsets) in offsets_pos.iter().enumerate() {
        // 获取当前节点的坐标信息
        let (l_idx, x, y) = get_coord(*offsets, &map_size, &offsets_defaults)?;
        // 获取当前节点的邻居节点
        let neighbours = get_neighbours(l_idx, x, y, &map_size, &offsets_defaults)?;
        // 遍历所有邻居节点
        for n_idx in 0..neighbours.len() {
            let noffsets = neighbours[n_idx];
            // 获取连接值
            let link_value = link_mask[noffsets.1];
            // 获取邻居节点类别
            let node_cls = node_mask[noffsets.0];
            // 如果连接有效且邻居节点有效
            if link_value == 1 && node_cls == 1 {
                pos_link += 1;
                // 合并当前节点和邻居节点
                let mut fixed_group_mask = group_mask.clone().into_dimensionality::<Ix1>().unwrap();
                join(&mut fixed_group_mask, *offsets, noffsets.0);
            }
        }
    }

    // 获取最终的分组结果
    let fixed_group_mask = group_mask.clone().into_dimensionality::<Ix1>().unwrap();
    let mask = get_all(&fixed_group_mask, &offsets_pos);
    Ok(mask)
}

// 获取坐标
pub fn get_coord(
    // 偏移量
    offsets: usize, 
    // 地图尺寸，每个元素是一个元组，分别表示宽度和高度
    map_size: &Vec<(usize, usize)>, 
    // 默认偏移量，用于确定当前偏移量属于哪个层级
    offsets_defaults: &Vec<(usize, usize)>, 
) -> Result<(usize, usize, usize)> {
    // 判断偏移量属于哪个层级，并计算对应的坐标
    if offsets < offsets_defaults[1].0 {
        // 当前偏移量属于第一个层级
        // 层级索引
        let l_idx = 0; 
        // 计算x坐标
        let x = offsets % map_size[0].1; 
        // 计算y坐标
        let y = offsets / map_size[0].1; 
        // 返回层级索引、x坐标和y坐标
        Ok((l_idx, x, y)) 
    } else if offsets < offsets_defaults[2].0 {
        // 当前偏移量属于第二个层级
        // 层级索引
        let l_idx = 1; 
        // 计算x坐标
        let x = (offsets - offsets_defaults[1].0) % map_size[1].1; 
        // 计算y坐标
        let y = (offsets - offsets_defaults[1].0) / map_size[1].1; 
        // 返回层级索引、x坐标和y坐标
        Ok((l_idx, x, y)) 
    } else if offsets < offsets_defaults[3].0 {
        // 当前偏移量属于第三个层级
        // 层级索引
        let l_idx = 2; 
        // 计算x坐标
        let x = (offsets - offsets_defaults[2].0) % map_size[2].1; 
        // 计算y坐标
        let y = (offsets - offsets_defaults[2].0) / map_size[2].1; 
        // 返回层级索引、x坐标和y坐标
        Ok((l_idx, x, y)) 
    } else if offsets < offsets_defaults[4].0 {
        // 当前偏移量属于第四个层级
        // 层级索引
        let l_idx = 3; 
        // 计算x坐标
        let x = (offsets - offsets_defaults[3].0) % map_size[3].1; 
        // 计算y坐标
        let y = (offsets - offsets_defaults[3].0) / map_size[3].1; 
        // 返回层级索引、x坐标和y坐标
        Ok((l_idx, x, y))
    } else if offsets < offsets_defaults[5].0 {
        // 当前偏移量属于第五个层级
        // 层级索引
        let l_idx = 4; 
        // 计算x坐标
        let x = (offsets - offsets_defaults[4].0) % map_size[4].1; 
        // 计算y坐标
        let y = (offsets - offsets_defaults[4].0) / map_size[4].1; 
        // 返回层级索引、x坐标和y坐标
        Ok((l_idx, x, y)) 
    } else {
        // 当前偏移量属于第六个层级
        // 层级索引
        let l_idx = 5; 
        // 计算x坐标
        let x = (offsets - offsets_defaults[5].0) % map_size[5].1; 
        // 计算y坐标
        let y = (offsets - offsets_defaults[5].0) / map_size[5].1; 
        // 返回层级索引、x坐标和y坐标
        Ok((l_idx, x, y)) 
    }
}

// 定义一个函数，用于获取指定位置的所有邻居节点信息
pub fn get_neighbours(
    // 当前层级索引
    l_idx: usize,
    // 当前节点的x坐标
    x: usize,
    // 当前节点的y坐标
    y: usize,
    // 各层级地图尺寸的向量，存储每个层级的(宽度,高度)
    map_size: &Vec<(usize, usize)>,
    // 各层级的偏移量默认值向量，存储每个层级的(节点偏移,链接偏移)
    offsets_defaults: &Vec<(usize, usize)>,
) -> Result<Vec<(usize, usize, usize)>> {
    // 初始化邻居坐标向量
    let mut coord: Vec<(usize, usize, usize)>;
    
    // 判断是否为最底层(第0层)
    if l_idx == 0 {
        // 如果是第0层，计算8个相邻网格坐标(使用wrapping避免溢出)
        coord = vec![
            // 左上
            (0, x.wrapping_sub(1), y.wrapping_sub(1)), 
            // 上 
            (0, x, y.wrapping_sub(1)),          
            // 右上        
            (0, x.wrapping_add(1), y.wrapping_sub(1)),  
            // 左
            (0, x.wrapping_sub(1), y),          
            // 右        
            (0, x.wrapping_add(1), y),                  
            // 左下
            (0, x.wrapping_sub(1), y.wrapping_add(1)),  
            // 下
            (0, x, y.wrapping_add(1)),            
            // 右下      
            (0, x.wrapping_add(1), y.wrapping_add(1)),  
        ];
    } else {
        // 如果不是第0层，计算12个相邻坐标(8个同层+4个下层)
        coord = vec![
            // 同层8个相邻网格
            (l_idx, x.wrapping_sub(1), y.wrapping_sub(1)),
            (l_idx, x, y.wrapping_sub(1)),
            (l_idx, x.wrapping_add(1), y.wrapping_sub(1)),
            (l_idx, x.wrapping_sub(1), y),
            (l_idx, x.wrapping_add(1), y),
            (l_idx, x.wrapping_sub(1), y.wrapping_add(1)),
            (l_idx, x, y.wrapping_add(1)),
            (l_idx, x.wrapping_add(1), y.wrapping_add(1)),
            // 下层4个子网格
            (l_idx.wrapping_sub(1), 2 * x, 2 * y),
            (l_idx.wrapping_sub(1), 2 * x + 1, 2 * y),
            (l_idx.wrapping_sub(1), 2 * x, 2 * y + 1),
            (l_idx.wrapping_sub(1), 2 * x + 1, 2 * y + 1),
        ];
    }

    // 初始化邻居偏移量向量
    let mut neighbours_offsets = Vec::new();
    // 初始化链接索引
    let mut link_idx = 0;
    
    // 遍历所有计算出的坐标
    for (nl_idx, nx, ny) in coord {
        // 检查坐标是否有效
        if is_valid_coord(nl_idx, nx, ny, map_size)? {
            // 计算节点在全局数组中的偏移量
            let neighbours_offset_node = offsets_defaults[nl_idx].0 + map_size[nl_idx].1 * ny + nx;
            
            // 如果是第0层
            if l_idx == 0 {
                // 计算链接在全局数组中的偏移量(8个本地链接)
                let neighbours_offset_link = offsets_defaults[l_idx].1 + (map_size[l_idx].1 * y + x) * N_LOCAL_LINKS + link_idx;
                // 将(节点偏移,链接偏移,链接索引)加入结果向量
                neighbours_offsets.push((neighbours_offset_node, neighbours_offset_link, link_idx));
            } else {
                // 如果不是第0层(8本地+4跨层链接)
                let off_tmp = (map_size[l_idx].1 * y + x) * (N_LOCAL_LINKS + N_CROSS_LINKS);
                // 计算链接在全局数组中的偏移量
                let neighbours_offset_link = offsets_defaults[l_idx].1 + off_tmp + link_idx;
                // 将(节点偏移,链接偏移,链接索引)加入结果向量
                neighbours_offsets.push((neighbours_offset_node, neighbours_offset_link, link_idx));
            }
        }
        // 递增链接索引
        link_idx += 1;
    }
    
    // 返回结果向量，包含每个有效邻居的(节点偏移,链接偏移,链接索引)
    Ok(neighbours_offsets)
}

// 检查坐标有效性
pub fn is_valid_coord(
    l_idx: usize,
    x: usize,
    y: usize,
    map_size: &Vec<(usize, usize)>,
) -> Result<bool> {
    let w = map_size[l_idx].1;
    let h = map_size[l_idx].0;
    Ok(x < w && y < h)
}

// 组合线段
pub fn combine_segments_python(
    segments: Array2<f32>,
    group_indices: Array1<i32>,
    segment_counts: Array1<i32>,
) -> Result<(Array2<f32>, Array1<i32>)> {
    // 添加批次维度，将segments转换为三维数组
    let segments_batch = segments.clone().to_shape((1, segments.nrows(), segments.ncols()))?.to_owned();
    // 添加批次维度，将group_indices转换为二维数组
    let group_indices_batch = group_indices.clone().to_shape((1, group_indices.len()))?.to_owned();
    // 调用combine_segments_batch函数
    let (combined_rboxes, combined_counts) = combine_segments_batch(segments_batch, group_indices_batch, segment_counts)?;
    Ok((combined_rboxes, combined_counts))
}

// 定义一个函数，用于批量合并线段
pub fn combine_segments_batch(
    // 输入参数：三维数组，表示批量线段数据
    segments_batch: Array3<f32>,
    // 输入参数：二维数组，表示分组索引
    group_indices_batch: Array2<i32>,
    // 输入参数：一维数组，表示每个图像的线段计数
    segment_counts_batch: Array1<i32>,
) -> Result<(Array2<f32>, Array1<i32>)> {
    // 设置批量大小为1（这里应该是可以调整的参数）
    let batch_size = 1;
    // 创建空向量，用于存储合并后的旋转矩形框
    let mut combined_rboxes_batch : Vec<Vec<Array1<f32>>> = Vec::new();
    // 创建空向量，用于存储合并后的计数
    let mut combined_counts_batch : Vec<i32> = Vec::new();

    // 遍历每张图像
    for image_id in 0..batch_size {
        // 获取当前图像的分组数量
        let group_count = segment_counts_batch[image_id] as usize;
        // 切片获取当前图像的所有线段
        let segments = segments_batch.slice(s![image_id, .., ..]);
        // 切片获取当前图像的分组索引
        let group_indices = group_indices_batch.slice(s![image_id, ..]);

        // 创建临时向量存储合并后的矩形框
        let mut combined_rboxes : Vec<Array1<f32>> = Vec::new();

        // 遍历每个分组
        for i in 0..group_count {
            // 获取属于当前分组的线段索引
            let indices: Vec<_> = group_indices.iter()
                .enumerate()
                .filter(|&(_, &val)| val == i as i32)
                .map(|(idx, _)| idx)
                .collect();
            
            // 检查分组是否有线段
            if !indices.is_empty() {
                // 使用select方法获取对应分组的线段
                let segments_group = segments.select(Axis(0), &indices);
                // 合并线段并获取旋转矩形框
                let combined_rbox = combine_segs(segments_group.into_owned())?;
                // 将结果存入临时向量
                combined_rboxes.push(combined_rbox);
            }
        }

        // 将当前图像的合并结果存入批量向量
        combined_rboxes_batch.push(combined_rboxes.clone());
        // 记录当前图像的合并结果数量
        combined_counts_batch.push(combined_rboxes.len() as i32);

    } // end for image_id

    // 找出最大的合并结果数量（用于填充）
    let max_count = *combined_counts_batch.iter().max().unwrap();

    // 对每张图像的合并结果进行填充，使其数量一致
    // 填充时保持一维数组结构
    for image_id in 0..batch_size {
        let current_count = combined_rboxes_batch[image_id].len();
        if current_count < max_count.try_into().unwrap() {
            let pad_size = max_count as usize - current_count;
            let pad_array = Array1::zeros(RBOX_DIM);
            combined_rboxes_batch[image_id].extend(vec![pad_array; pad_size.try_into().unwrap()]);
        }
    }

    // 将批量结果转换为动态维度的数组
    let mut result_array = Array2::zeros(Dim::<[usize; 2]>::new([batch_size, ((max_count as usize) * RBOX_DIM ) as usize]));
    for (i, rboxes) in combined_rboxes_batch.iter().enumerate() {
        let flat_data: Vec<f32> = rboxes.iter()
            .flat_map(|arr| arr.iter().cloned())
            .collect();
        let row = Array1::from(flat_data);
        result_array.slice_mut(s![i, ..]).assign(&row);
    }
    // 将计数结果转换为动态维度的数组
    let counts_array = Array1::from(combined_counts_batch);

    // 返回合并后的旋转矩形框和计数
    Ok((result_array, counts_array))

}

// 融合分割结果
pub fn combine_segs(segs: Array2<f32>) -> Result<Array1<f32>> {
    // 断言输入的segs是二维数组
    assert_eq!(segs.ndim(), 2, "invalid segs ndim");
    // 断言输入的segs的第二维大小为6
    assert_eq!(segs.shape()[1], 6, "invalid segs shape");

    // 如果segs只有一行，直接计算并返回结果
    if segs.shape()[0] == 1 {
        // 提取segs中的各个值
        let cx = segs[(0, 0)];
        let cy = segs[(0, 1)];
        let w = segs[(0, 2)];
        let h = segs[(0, 3)];
        let theta_sin = segs[(0, 4)];
        let theta_cos = segs[(0, 5)];
        // 计算角度
        let theta = theta_sin.atan2(theta_cos);
        // 返回结果
        return Ok(Array::from_vec(vec![cx, cy, w, h, theta]));
    }

    // 提取所有中心点的x坐标
    let cxs = CowArray::from(segs.slice(s![.., 0]));
    // 提取所有中心点的y坐标
    let cys = CowArray::from(segs.slice(s![.., 1]));

    // 提取所有theta的cos值
    let theta_coss = segs.slice(s![.., 4]);
    // 提取所有theta的sin值
    let theta_sins = segs.slice(s![.., 5]);

    // 计算平均角度的tan值
    let bar_theta = theta_sins.sum() / theta_coss.sum();
    // 计算直线的斜率
    let k = bar_theta.tan();
    // 计算直线的截距
    let b = (cys.clone() - k * cxs.clone()).sum() / cxs.len() as f32;

    // 计算所有点在直线上的投影x坐标
    let proj_xs = (k * cys.clone() + cxs.clone() - k * b) / (k.powi(2) + 1.0);
    // 计算所有点在直线上的投影y坐标
    let proj_ys = (k.powi(2) * cys.clone() + k * cxs.clone() + b) / (k.powi(2) + 1.0);
    // 将投影点的x和y坐标组合成一个二维数组
    let proj_points = stack!(Axis(1), &[proj_xs.clone(), proj_ys.clone()]);

    // 初始化最大距离和对应的索引
    let mut max_dist = -1.0;
    let mut idx1 = -1;
    let mut idx2 = -1;

    // 遍历所有投影点，找到距离最大的两个点
    for i in 0..proj_xs.len() {
        let x1 = proj_xs[i];
        let y1 = proj_ys[i];
        for j in i + 1..proj_xs.len() {
            let x2 = proj_xs[j];
            let y2 = proj_ys[j];
            // 计算欧氏距离
            let dx = x1 - x2;
            let dy = y1 - y2;
            let dist = (dx * dx + dy * dy).sqrt();
            if dist > max_dist {
                idx1 = i as i32;
                idx2 = j as i32;
                max_dist = dist;
            }
        }
    }

    // unimplemented!()

    // 确保找到了两个有效的索引
    assert!(idx1 >= 0 && idx2 >= 0);

    // 提取两个最远点对应的segs行（假设segs是二维数组）
    // seg1 是一维数组
    let seg1 = segs.slice(s![idx1 as usize, ..]); 
    // seg2 是一维数组
    let seg2 = segs.slice(s![idx2 as usize, ..]); 

    // 计算中心坐标和尺寸（假设每行格式为 [x, y, width, height]）
    // 直接索引第一个元素（x坐标）
    let bcx = (seg1[0] + seg2[0]) / 2.0; 
    // 直接索引第二个元素（y坐标）
    let bcy = (seg1[1] + seg2[1]) / 2.0; 
    let bh = segs.slice(s![.., 3]).sum() / segs.shape()[0] as f32;
    // 直接索引第三个元素（宽度）
    let bw = max_dist + (seg1[2] + seg2[2]) / 2.0; 

    // 返回最终结果
    Ok(Array::from_vec(vec![bcx, bcy, bw, bh, bar_theta]))
}

// 定义非极大值抑制函数，输入为一个二维向量，表示边界框的坐标和分数
pub fn nms_python(boxes: Vec<Vec<f32>>) -> Vec<Vec<f32>> {
    // 将输入的边界框复制一份，并转换为可变向量
    let mut boxes = boxes.into_iter().collect::<Vec<Vec<f32>>>();
    // 按照边界框的分数（第9个元素）从大到小进行排序
    // 使用 sort_by 和 partial_cmp 来对 f32 类型进行排序
    boxes.sort_by(|a, b| b[8].partial_cmp(&a[8]).unwrap());

    // 初始化一个布尔向量，用于标记每个边界框是否保留，初始全部为true
    let mut nms_flag = vec![true; boxes.len()];
    // 遍历所有边界框
    for (i, a) in boxes.iter().enumerate() {
        // 如果当前边界框已经被标记为不保留，则跳过
        if!nms_flag[i] {
            continue;
        } else {
            // 遍历当前边界框之后的所有边界框
            for (j, b) in boxes.iter().enumerate().filter(|(j, _)| *j > i) {
                // 如果当前边界框已经被标记为不保留，则跳过
                if!nms_flag[j] {
                    continue;
                }
                // 获取当前边界框和另一个边界框的分数
                let score_a = a[8];
                let score_b = b[8];
                // 将边界框的前8个坐标转换为旋转框
                let rbox_a = polygon2rbox(&a[..8]);
                let rbox_b = polygon2rbox(&b[..8]);
                // 判断两个旋转框的中心点是否在对方的旋转框内
                if point_in_rbox(&rbox_a[..2], &rbox_a) || point_in_rbox(&rbox_b[..2], &rbox_b) {
                    // 如果当前边界框的分数大于另一个边界框的分数，则将另一个边界框标记为不保留
                    if score_a > score_b {
                        nms_flag[j] = false;
                    }
                }
            }
        }
    }

    // 过滤出保留的边界框并返回
    boxes.into_iter()
       .enumerate()
       .filter(|(i, _)| nms_flag[*i])
       .map(|(_, _box)| _box)
       .collect()
}

// 定义一个函数，将多边形转换为旋转矩形
pub fn polygon2rbox(polygon: &[f32]) -> Vec<f32> {
    // 提取多边形的四个顶点坐标
    let x1 = polygon[0];
    let x2 = polygon[2];
    let x3 = polygon[4];
    let x4 = polygon[6];
    let y1 = polygon[1];
    let y2 = polygon[3];
    let y3 = polygon[5];
    let y4 = polygon[7];

    // 计算旋转矩形的中心点坐标
    let c_x = (x1 + x2 + x3 + x4) / 4.0;
    let c_y = (y1 + y2 + y3 + y4) / 4.0;

    // 计算旋转矩形的宽度和高度
    // 计算第一条边的长度
    let w1 = point_dist(x1, y1, x2, y2); 
    // 计算第三条边的长度
    let w2 = point_dist(x3, y3, x4, y4); 
    // 计算中心点到第一条边的距离
    let h1 = point_line_dist(c_x, c_y, x1, y1, x2, y2); 
    // 计算中心点到第三条边的距离
    let h2 = point_line_dist(c_x, c_y, x3, y3, x4, y4); 
    // 旋转矩形的高度为两个距离之和
    let h = h1 + h2; 
    // 旋转矩形的宽度为两条边长度的平均值
    let w = (w1 + w2) / 2.0; 

    // 计算旋转矩形的旋转角度
    // 计算第一条边的斜率角度
    let theta1 = (y2 - y1).atan2(x2 - x1); 
    // 计算第三条边的斜率角度
    let theta2 = (y3 - y4).atan2(x3 - x4); 
    // 旋转矩形的角度为两个角度的平均值
    let theta = (theta1 + theta2) / 2.0; 

    // 返回旋转矩形的参数
    vec![c_x, c_y, w, h, theta]
}

// 计算宽度
pub fn cal_width(_box: &[f32]) -> f32 {
    let pd1 = point_dist(_box[0], _box[1], _box[2], _box[3]);
    let pd2 = point_dist(_box[4], _box[5], _box[6], _box[7]);
    (pd1 + pd2) / 2.0
}

// 计算点距离
pub fn point_dist(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
}

// 定义一个函数，判断一个点是否在旋转矩形内
pub fn point_in_rbox(c: &[f32], rbox: &[f32]) -> bool {
    // 获取点的坐标
    let cx0 = c[0];
    let cy0 = c[1];
    // 获取旋转矩形的中心坐标、宽、高和旋转角度
    let cx1 = rbox[0];
    let cy1 = rbox[1];
    let w = rbox[2];
    let h = rbox[3];
    let theta = rbox[4];

    // 计算点到旋转矩形中心在旋转角度下的水平方向距离
    let dist_x = ((cx1 - cx0) * theta.cos() + (cy1 - cy0) * theta.sin()).abs();
    // 计算点到旋转矩形中心在旋转角度下的垂直方向距离
    let dist_y = (-(cx1 - cx0) * theta.sin() + (cy1 - cy0) * theta.cos()).abs();

    // 判断点是否在旋转矩形内，即水平和垂直方向距离是否都小于矩形对应方向的一半
    dist_x < w / 2.0 && dist_y < h / 2.0
}

// 定义一个函数，用于计算点到直线的距离
pub fn point_line_dist(px: f32, py: f32, x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    // 定义一个小的常量值，用于避免除以零的情况
    let eps = 1e-6;
    // 计算直线的水平方向增量
    let dx = x2 - x1;
    // 计算直线的垂直方向增量
    let dy = y2 - y1;
    // 计算直线的长度，加上 eps 避免除零错误
    let div = (dx.powi(2) + dy.powi(2)).sqrt() + eps;
    // 计算点到直线的距离公式
    ((px * dy - py * dx + x2 * y1 - y2 * x1).abs() / div)
}

// 定义一个函数，将旋转矩形框（rboxes）转换为多边形（polygons）
pub fn rboxes_to_polygons(rboxes: Array2<f32>) -> Array4<f32> {
    // 提取旋转角度
    let theta = rboxes.slice(s![.., 4..5]).to_owned();
    // 提取矩形框的中心点坐标
    let cxcy = rboxes.slice(s![.., 0..2]).to_owned();
    // 提取矩形框的宽度并除以2，得到半宽
    let half_w = rboxes.slice(s![.., 2..3]).mapv(|x| x / 2.0);
    // 提取矩形框的高度并除以2，得到半高
    let half_h = rboxes.slice(s![.., 3..4]).mapv(|x| x / 2.0);

    // 计算向量v1，表示沿宽度方向的偏移量
    let v1 = stack![Axis(1), theta.clone() * half_w.clone(), theta.clone() * half_w.clone()];
    // 计算向量v2，表示沿高度方向的偏移量
    let v2 = stack![Axis(1), -(theta.clone() * half_h.clone()), theta.clone() * half_h.clone()];

    // 计算多边形的第一个顶点坐标
    let p1 = cxcy.clone() - v1.clone() - v2.clone();
    // 计算多边形的第二个顶点坐标
    let p2 = cxcy.clone() + v1.clone() - v2.clone();
    // 计算多边形的第三个顶点坐标
    let p3 = cxcy.clone() + v1.clone() + v2.clone();
    // 计算多边形的第四个顶点坐标
    let p4 = cxcy.clone() - v1.clone() + v2.clone();

    // 将四个顶点坐标堆叠起来，形成最终的多边形数组并返回
    stack![Axis(1), p1, p2, p3, p4]
}