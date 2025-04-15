# 使用dgocr识别字母

# 识别rec:
# 输入:
# input_images
# name: input_images
# tensor: float32[batch_size,channel,height,width]
# 输出:
# name: result
# tensor: float32[batch_size,length,class]
# 检测det:
# 输入:
# input_images
# name: input_images
# tensor: float32[1,3,1024,1024]
# 输出:
# name: node_labels_0
# tensor: float32[1,256,256]
# name: link_labels_0
# tensor: float32[1,256,256,8]
# name: reg_maps_0
# tensor: float32[1,256,256,6]

#!/usr/bin/env python
# -*- coding:utf-8 -*-

import cv2
import numpy as np
from PIL import Image

# 引入自定义的文字识别模块
from .rec import DGOCRRecognition
# 引入自定义的文本框检测模块
from .det import DGOCRDetection
# 引入自定义的基于SegLink的文本检测模块
from .det_seglink import SegLinkOCRDetection
# 引入自定义的可视化模块
from .visual import draw_ocr_box_txt
# 引入自定义的工具模块
from .utils import crop_image, order_point, preprocess, postprocess

# 定义常量
OFFSET_DIM = 6
RBOX_DIM = 5
N_LOCAL_LINKS = 8
N_CROSS_LINKS = 4
N_SEG_CLASSES = 2
N_LNK_CLASSES = 4
MATCH_STATUS_POS = 1
MATCH_STATUS_NEG = -1
MATCH_STATUS_IGNORE = 0
MUT_LABEL = 3
POS_LABEL = 1
NEG_LABEL = 0
N_DET_LAYERS = 6
FLAGS_NODE_THRESHOLD = 0.4
FLAGS_LINK_THRESHOLD = 0.6

# 定义DGOCR类，用于OCR识别
class DGOCR:
    def __init__(self, rec_path, det_path, img_size=1600, cpu_thread_num=2, model_type="common") -> None:
        """
        初始化模型

        Args:
            rec_path (str): 文字识别模型文件夹路径
            det_path (str): 文本框检测模型文件路径
            img_size (int): 模型限定的图像大小. 默认值为 1600.
            cpu_thread_num (int): CPU线程数, 默认为 2. 越大速度越快，但占用资源越多
            model_type (str): 模型类型, 默认为 "common" 生成模型, 可选 "seglink" 推理模型.
        """
        self.rec_path = rec_path
        self.det_path = det_path
        self.img_size = img_size
        self.cpu_thread_num = cpu_thread_num
        self.model_type = model_type
        # 加载模型
        self.load_model()

    def load_model(self):
        # 加载文字识别模型
        self.rec_model = DGOCRRecognition(self.rec_path, self.cpu_thread_num)
        # 根据模型类型加载文本框检测模型
        if self.model_type == "seglink":
            self.det_model = SegLinkOCRDetection(self.det_path, self.cpu_thread_num)
        else:
            self.det_model = DGOCRDetection(self.det_path, self.img_size, self.cpu_thread_num)

    def run(self, image):
        """
        运行模型

        Args:
            image (str): 图像路径

        Returns:
            ocr_result: 识别结果, [[box, score, text],...]; box 为文本框四个点坐标, score 为文本框的置信度, text 为识别文本
        """
        # 读取原始图像
        original_image = cv2.imread(image)
        # 获取原始图像的尺寸
        original_image_size = original_image.shape[:2]

        # 进行文本框检测
        if self.model_type == "seglink":
            det_result = self.det_model.run(original_image)
        else:
            # 图片预处理
            image_full = preprocess(original_image, (self.img_size, self.img_size))
            # 获取当前图像的尺寸
            current_image_size = image_full.shape[:2]
            det_result = self.det_model.run(image_full)

        # 获取检测到的文本框多边形坐标
        boxes = np.array(det_result['polygons'])

        # 初始化位置列表、文本列表和得分列表
        pos_list = []
        text_list = []
        score_list = []
        for i in range(boxes.shape[0]):
            # 对文本框的点进行排序
            pts = order_point(boxes[i])
            if self.model_type == "seglink":
                # 裁剪文本框对应的图像
                image_crop = crop_image(original_image, pts)
            else:
                image_crop = crop_image(image_full, pts)
            # 进行文字识别
            result = self.rec_model.run(image_crop)
            if len(result[0]) > 0:
                # 将排序后的点添加到位置列表
                pos_list.append(pts.tolist())
                # 将识别到的文本添加到文本列表
                text_list.append(result[0][0])
                # 将识别得分添加到得分列表
                score_list.append(result[1][0])

        # 后处理
        if self.model_type != "seglink":
            pos_list = postprocess(original_image_size, current_image_size, pos_list)
        # 初始化OCR结果列表
        ocr_result = []
        for i in range(len(pos_list)):
            if len(pos_list[i]) > 0:
                # 将位置、得分和文本组合成一个结果添加到OCR结果列表
                ocr_result.append([pos_list[i], (text_list[i], score_list[i])])

        return ocr_result

    def draw(self, img_path, ocr_result, save_path):
        """
        绘制识别结果

        Args:
            image (str): 图像路径
            ocr_result (list): 识别结果
            save_path (str): 保存路径
        """
        # 打开图像并转换为RGB格式
        image = Image.open(img_path).convert('RGB')
        # 从ocr_result中获取文本框、得分和文本
        boxs = [i[0] for i in ocr_result]
        texts = [i[1][0] for i in ocr_result]
        # 绘制识别结果
        image = draw_ocr_box_txt(image, boxs, texts)
        # 将绘制结果转换为PIL图像
        im_show = Image.fromarray(image)
        # 保存绘制结果
        im_show.save(save_path)

# decode_segments_links_python 解码
def decode_segments_links_python(image_size, all_nodes, all_links, all_reg, anchor_sizes):
    # 批量大小为1
    batch_size = 1
    # 将输入的数据展平并拼接
    all_nodes_flat = np.concatenate([np.reshape(o, (batch_size, -1, N_SEG_CLASSES)) for o in all_nodes], axis=1)
    all_links_flat = np.concatenate([np.reshape(o, (batch_size, -1, N_LNK_CLASSES)) for o in all_links], axis=1)
    all_reg_flat = np.concatenate([np.reshape(o, (batch_size, -1, OFFSET_DIM)) for o in all_reg], axis=1)
    # 使用decode_batch函数解码
    segments, group_indices, segment_counts, group_indices_all = decode_batch(
        all_nodes_flat, all_links_flat, all_reg_flat, image_size, np.array(anchor_sizes)
    )
    return segments, group_indices, segment_counts, group_indices_all

# 批量解码函数
def decode_batch(all_nodes, all_links, all_reg, image_size, anchor_sizes):
    # 获取批量大小
    batch_size = all_nodes.shape[0]
    # 初始化批量的线段、组索引、线段计数和组索引全量列表
    batch_segments = []
    batch_group_indices = []
    batch_segments_counts = []
    batch_group_indices_all = []
    for image_id in range(batch_size):
        # 获取当前图像的节点得分、链接得分和回归值
        image_node_scores = all_nodes[image_id, :, :]
        image_link_scores = all_links[image_id, :, :]
        image_reg = all_reg[image_id, :, :]
        # 对当前图像进行解码
        image_segments, image_group_indices, image_segments_counts, image_group_indices_all = decode_image(
            image_node_scores, image_link_scores, image_reg, image_size, anchor_sizes
        )
        # 将当前图像的解码结果添加到批量列表中
        batch_segments.append(image_segments)
        batch_group_indices.append(image_group_indices)
        batch_segments_counts.append(image_segments_counts)
        batch_group_indices_all.append(image_group_indices_all)
    # 获取最大的线段计数
    max_count = np.max(batch_segments_counts)
    for image_id in range(batch_size):
        if not batch_segments_counts[image_id] == max_count:
            # 对不足最大计数的线段进行填充
            batch_segments_pad = (max_count - batch_segments_counts[image_id]) * [OFFSET_DIM * [0.0]]
            batch_segments[image_id] = np.vstack((batch_segments[image_id], np.array(batch_segments_pad)))
            # 对不足最大计数的组索引进行填充
            batch_group_indices[image_id] = np.hstack((batch_group_indices[image_id],
                                                       np.array((max_count - batch_segments_counts[image_id]) * [-1])))
    # 将批量列表转换为numpy数组
    a = np.asarray(batch_segments, np.float32)
    b = np.asarray(batch_group_indices, np.int32)
    c = np.asarray(batch_segments_counts, np.int32)
    d = np.asarray(batch_group_indices_all, np.int32)
    return a, b, c, d

# 单张图像解码函数
def decode_image(image_node_scores, image_link_scores, image_reg, image_size, anchor_sizes):
    # 初始化特征图大小和默认偏移量列表
    map_size = []
    offsets_defaults = []
    offsets_default_node = 0
    offsets_default_link = 0

    for i in range(N_DET_LAYERS):
        # 记录当前层的默认偏移量
        offsets_defaults.append([offsets_default_node, offsets_default_link])
        # 计算当前层的特征图大小
        map_size.append(image_size // (2 ** (2 + i)))
        # 更新节点默认偏移量
        offsets_default_node += map_size[i][0] * map_size[i][1]
        if i == 0:
            # 更新第一层的链接默认偏移量
            offsets_default_link += map_size[i][0] * map_size[i][1] * N_LOCAL_LINKS
        else:
            # 更新其他层的链接默认偏移量
            offsets_default_link += map_size[i][0] * map_size[i][1] * (N_LOCAL_LINKS + N_CROSS_LINKS)

    # 通过连接方式对图像进行解码
    image_group_indices_all = decode_image_by_join(image_node_scores,
                                                   image_link_scores,
                                                   FLAGS_NODE_THRESHOLD,
                                                   FLAGS_LINK_THRESHOLD,
                                                   map_size, offsets_defaults)
    # 调整组索引
    image_group_indices_all -= 1
    # 获取有效的组索引
    image_group_indices = image_group_indices_all[np.where(image_group_indices_all >= 0)[0]]
    # 计算线段数量
    image_segments_counts = len(image_group_indices)
    # 初始化线段数组
    image_segments = np.zeros((image_segments_counts, OFFSET_DIM), dtype=np.float32)
    for i, offsets in enumerate(np.where(image_group_indices_all >= 0)[0]):
        # 获取编码的中心点坐标、宽度、高度、角度余弦和正弦值
        encoded_cx = image_reg[offsets, 0]
        encoded_cy = image_reg[offsets, 1]
        encoded_width = image_reg[offsets, 2]
        encoded_height = image_reg[offsets, 3]
        encoded_theta_cos = image_reg[offsets, 4]
        encoded_theta_sin = image_reg[offsets, 5]

        # 获取当前点的层索引、x坐标和y坐标
        l_idx, x, y = get_coord(offsets, map_size, offsets_defaults)
        # 获取当前层的锚点大小
        rs = anchor_sizes[l_idx]
        # 定义一个小的常量
        eps = 1e-6
        # 计算线段的中心点坐标、宽度、高度、角度余弦和正弦值
        image_segments[i, 0] = encoded_cx * rs + (2 ** (2 + l_idx)) * (x + 0.5)
        image_segments[i, 1] = encoded_cy * rs + (2 ** (2 + l_idx)) * (y + 0.5)
        image_segments[i, 2] = np.exp(encoded_width) * rs - eps
        image_segments[i, 3] = np.exp(encoded_height) * rs - eps
        image_segments[i, 4] = encoded_theta_cos
        image_segments[i, 5] = encoded_theta_sin

    return image_segments, image_group_indices, image_segments_counts, image_group_indices_all

# 通过连接方式对图像进行解码
def decode_image_by_join(node_scores, link_scores, node_threshold,
                         link_threshold, map_size, offsets_defaults):
    # 获取节点得分大于阈值的掩码
    node_mask = node_scores[:, POS_LABEL] >= node_threshold
    # 获取链接得分大于阈值的掩码
    link_mask = link_scores[:, POS_LABEL] >= link_threshold
    # 初始化组掩码
    group_mask = np.zeros_like(node_mask, np.int32) - 1
    # 获取节点得分大于阈值的位置
    offsets_pos = np.where(node_mask == 1)[0]

    # 查找节点的父节点
    def find_parent(point):
        return group_mask[point]

    # 设置节点的父节点
    def set_parent(point, parent):
        group_mask[point] = parent

    # 判断节点是否为根节点
    def is_root(point):
        return find_parent(point) == -1

    # 查找节点的根节点
    def find_root(point):
        root = point
        update_parent = False
        while not is_root(root):
            root = find_parent(root)
            update_parent = True

        # 加速查找根节点
        if update_parent:
            set_parent(point, root)

        return root

    # 合并两个节点
    def join(p1, p2):
        root1 = find_root(p1)
        root2 = find_root(p2)

        if root1 != root2:
            set_parent(root1, root2)

    # 获取所有节点的组索引
    def get_all():
        root_map = {}

        def get_index(root):
            if root not in root_map:
                root_map[root] = len(root_map) + 1
            return root_map[root]

        mask = np.zeros_like(node_mask, dtype=np.int32)
        for i, point in enumerate(offsets_pos):
            point_root = find_root(point)
            bbox_idx = get_index(point_root)
            mask[point] = bbox_idx
        return mask

    # 通过链接合并节点
    pos_link = 0
    for i, offsets in enumerate(offsets_pos):
        # 获取当前点的层索引、x坐标和y坐标
        l_idx, x, y = get_coord(offsets, map_size, offsets_defaults)
        # 获取当前点的邻居节点
        neighbours = get_neighbours(l_idx, x, y, map_size, offsets_defaults)
        for n_idx, noffsets in enumerate(neighbours):
            # 获取链接得分和节点得分
            link_value = link_mask[noffsets[1]]
            node_cls = node_mask[noffsets[0]]
            if link_value and node_cls:
                # 合并节点
                pos_link += 1
                join(offsets, noffsets[0])
    # 获取所有节点的组索引
    mask = get_all()
    return mask

# 获取点的坐标
def get_coord(offsets, map_size, offsets_defaults):
    if offsets < offsets_defaults[1][0]:
        l_idx = 0
        x = offsets % map_size[0][1]
        y = offsets // map_size[0][1]
    elif offsets < offsets_defaults[2][0]:
        l_idx = 1
        x = (offsets - offsets_defaults[1][0]) % map_size[1][1]
        y = (offsets - offsets_defaults[1][0]) // map_size[1][1]
    elif offsets < offsets_defaults[3][0]:
        l_idx = 2
        x = (offsets - offsets_defaults[2][0]) % map_size[2][1]
        y = (offsets - offsets_defaults[2][0]) // map_size[2][1]
    elif offsets < offsets_defaults[4][0]:
        l_idx = 3
        x = (offsets - offsets_defaults[3][0]) % map_size[3][1]
        y = (offsets - offsets_defaults[3][0]) // map_size[3][1]
    elif offsets < offsets_defaults[5][0]:
        l_idx = 4
        x = (offsets - offsets_defaults[4][0]) % map_size[4][1]
        y = (offsets - offsets_defaults[4][0]) // map_size[4][1]
    else:
        l_idx = 5
        x = (offsets - offsets_defaults[5][0]) % map_size[5][1]
        y = (offsets - offsets_defaults[5][0]) // map_size[5][1]

    return l_idx, x, y

# 获取点的邻居节点
def get_neighbours(l_idx, x, y, map_size, offsets_defaults):
    if l_idx == 0:
        coord = [(0, x - 1, y - 1), (0, x, y - 1), (0, x + 1, y - 1),
                 (0, x - 1, y), (0, x + 1, y), (0, x - 1, y + 1),
                 (0, x, y + 1), (0, x + 1, y + 1)]
    else:
        coord = [(l_idx, x - 1, y - 1),
                 (l_idx, x, y - 1), (l_idx, x + 1, y - 1), (l_idx, x - 1, y),
                 (l_idx, x + 1, y), (l_idx, x - 1, y + 1), (l_idx, x, y + 1),
                 (l_idx, x + 1, y + 1), (l_idx - 1, 2 * x, 2 * y),
                 (l_idx - 1, 2 * x + 1, 2 * y), (l_idx - 1, 2 * x, 2 * y + 1),
                 (l_idx - 1, 2 * x + 1, 2 * y + 1)]
    neighbours_offsets = []
    link_idx = 0
    for nl_idx, nx, ny in coord:
        if is_valid_coord(nl_idx, nx, ny, map_size):
            # 获取邻居节点的偏移量
            neighbours_offset_node = offsets_defaults[nl_idx][
                0] + map_size[nl_idx][1] * ny + nx
            if l_idx == 0:
                # 获取第一层邻居节点的链接偏移量
                neighbours_offset_link = offsets_defaults[l_idx][1] + (
                    map_size[l_idx][1] * y + x) * N_LOCAL_LINKS + link_idx
            else:
                # 获取其他层邻居节点的链接偏移量
                off_tmp = (map_size[l_idx][1] * y + x) * (
                    N_LOCAL_LINKS + N_CROSS_LINKS)
                neighbours_offset_link = offsets_defaults[l_idx][
                    1] + off_tmp + link_idx
            # 将邻居节点的偏移量添加到列表中
            neighbours_offsets.append(
                [neighbours_offset_node, neighbours_offset_link, link_idx])
        link_idx += 1
    # [节点偏移量, 链接偏移量, 链接索引(0-7/11)]
    return neighbours_offsets

# 判断坐标是否有效
def is_valid_coord(l_idx, x, y, map_size):
    w = map_size[l_idx][1]
    h = map_size[l_idx][0]
    return x >= 0 and x < w and y >= 0 and y < h

# 合并线段
def combine_segments_python(segments, group_indices, segment_counts):
    # 批量合并线段
    combined_rboxes, combined_counts = combine_segments_batch(segments, group_indices, segment_counts)
    return combined_rboxes, combined_counts

# 批量合并线段
def combine_segments_batch(segments_batch, group_indices_batch,
                           segment_counts_batch):
    # 批量大小为1
    batch_size = 1
    # 初始化批量合并后的旋转框和计数列表
    combined_rboxes_batch = []
    combined_counts_batch = []
    for image_id in range(batch_size):
        # 获取当前图像的线段数量
        group_count = segment_counts_batch[image_id]
        # 获取当前图像的线段
        segments = segments_batch[image_id, :, :]
        # 获取当前图像的组索引
        group_indices = group_indices_batch[image_id, :]
        # 初始化合并后的旋转框列表
        combined_rboxes = []
        for i in range(group_count):
            # 获取当前组的线段
            segments_group = segments[np.where(group_indices == i)[0], :]
            if segments_group.shape[0] > 0:
                # 合并当前组的线段
                combined_rbox = combine_segs(segments_group)
                # 将合并后的旋转框添加到列表中
                combined_rboxes.append(combined_rbox)
        # 将当前图像的合并后的旋转框添加到批量列表中
        combined_rboxes_batch.append(combined_rboxes)
        # 将当前图像的合并后的旋转框数量添加到批量列表中
        combined_counts_batch.append(len(combined_rboxes))

    # 获取最大的合并后的旋转框数量
    max_count = np.max(combined_counts_batch)
    for image_id in range(batch_size):
        if not combined_counts_batch[image_id] == max_count:
            # 对不足最大数量的合并后的旋转框进行填充
            combined_rboxes_pad = (max_count - combined_counts_batch[image_id]
                                   ) * [RBOX_DIM * [0.0]]
            combined_rboxes_batch[image_id] = np.vstack(
                (combined_rboxes_batch[image_id],
                 np.array(combined_rboxes_pad)))

    # 将批量列表转换为numpy数组
    return np.asarray(combined_rboxes_batch,
                      np.float32), np.asarray(combined_counts_batch, np.int32)

# 合并线段
def combine_segs(segs):
    segs = np.asarray(segs)
    assert segs.ndim == 2, '无效的线段维度'
    assert segs.shape[-1] == 6, '无效的线段形状'

    if len(segs) == 1:
        # 当只有一个线段时，直接返回该线段的旋转框
        cx = segs[0, 0]
        cy = segs[0, 1]
        w = segs[0, 2]
        h = segs[0, 3]
        theta_sin = segs[0, 4]
        theta_cos = segs[0, 5]
        theta = np.arctan2(theta_sin, theta_cos)
        return np.array([cx, cy, w, h, theta])

    # 找到所有中心点的最佳拟合直线: y = kx + b
    cxs = segs[:, 0]
    cys = segs[:, 1]

    theta_coss = segs[:, 4]
    theta_sins = segs[:, 5]

    # 计算平均角度
    bar_theta = np.arctan2(theta_sins.sum(), theta_coss.sum())
    # 计算直线斜率
    k = np.tan(bar_theta)
    # 计算直线截距
    b = np.mean(cys - k * cxs)

    # 计算中心点在直线上的投影点
    proj_xs = (k * cys + cxs - k * b) / (k ** 2 + 1)
    proj_ys = (k * k * cys + k * cxs + b) / (k ** 2 + 1)
    proj_points = np.stack((proj_xs, proj_ys), -1)

    # 找到最大距离
    max_dist = -1
    idx1 = -1
    idx2 = -1

    for i in range(len(proj_points)):
        point1 = proj_points[i, :]
        for j in range(i + 1, len(proj_points)):
            point2 = proj_points[j, :]
            dist = np.sqrt(np.sum((point1 - point2) ** 2))
            if dist > max_dist:
                idx1 = i
                idx2 = j
                max_dist = dist
    assert idx1 >= 0 and idx2 >= 0
    # 合并后的旋转框: 中心点坐标、宽度、高度、平均角度
    seg1 = segs[idx1, :]
    seg2 = segs[idx2, :]
    bcx, bcy = (seg1[:2] + seg2[:2]) / 2.0
    bh = np.mean(segs[:, 3])
    bw = max_dist + (seg1[2] + seg2[2]) / 2.0
    return bcx, bcy, bw, bh, bar_theta

# 计算宽度
def cal_width(box):
    # 计算相邻两点的距离
    pd1 = point_dist(box[0], box[1], box[2], box[3])
    pd2 = point_dist(box[4], box[5], box[6], box[7])
    return (pd1 + pd2) / 2

# 计算两点之间的距离
def point_dist(x1, y1, x2, y2):
    return np.sqrt((x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1))

# 非极大值抑制
def nms_python(boxes):
    # 按得分从高到低排序
    boxes = sorted(boxes, key=lambda x: -x[8])
    # 初始化非极大值抑制标志
    nms_flag = [True] * len(boxes)
    for i, a in enumerate(boxes):
        if not nms_flag[i]:
            continue
        else:
            for j, b in enumerate(boxes):
                if not j > i:
                    continue
                if not nms_flag[j]:
                    continue
                # 获取两个框的得分
                score_a = a[8]
                score_b = b[8]
                # 将多边形框转换为旋转框
                rbox_a = polygon2rbox(a[:8])
                rbox_b = polygon2rbox(b[:8])
                # 判断一个框的中心点是否在另一个框内
                if point_in_rbox(rbox_a[:2], rbox_b) or point_in_rbox(
                        rbox_b[:2], rbox_a):
                    if score_a > score_b:
                        # 如果得分高的框包含得分低的框，则抑制得分低的框
                        nms_flag[j] = False
    # 筛选出未被抑制的框
    boxes_nms = []
    for i, box in enumerate(boxes):
        if nms_flag[i]:
            boxes_nms.append(box)
    return boxes_nms

# 将多边形框转换为旋转框
def polygon2rbox(polygon):
    x1, x2, x3, x4 = polygon[0], polygon[2], polygon[4], polygon[6]
    y1, y2, y3, y4 = polygon[1], polygon[3], polygon[5], polygon[7]
    # 计算中心点坐标
    c_x = (x1 + x2 + x3 + x4) / 4
    c_y = (y1 + y2 + y3 + y4) / 4
    # 计算宽度
    w1 = point_dist(x1, y1, x2, y2)
    w2 = point_dist(x3, y3, x4, y4)
    # 计算高度
    h1 = point_line_dist(c_x, c_y, x1, y1, x2, y2)
    h2 = point_line_dist(c_x, c_y, x3, y3, x4, y4)
    h = h1 + h2
    w = (w1 + w2) / 2
    # 计算角度
    theta1 = np.arctan2(y2 - y1, x2 - x1)
    theta2 = np.arctan2(y3 - y4, x3 - x4)
    theta = (theta1 + theta2) / 2.0
    return [c_x, c_y, w, h, theta]

# 判断点是否在旋转框内
def point_in_rbox(c, rbox):
    cx0, cy0 = c[0], c[1]
    cx1, cy1 = rbox[0], rbox[1]
    w, h = rbox[2], rbox[3]
    theta = rbox[4]
    # 计算点到旋转框中心点的距离在旋转框坐标轴上的投影
    dist_x = np.abs((cx1 - cx0) * np.cos(theta) + (cy1 - cy0) * np.sin(theta))
    dist_y = np.abs(-(cx1 - cx0) * np.sin(theta) + (cy1 - cy0) * np.cos(theta))
    return ((dist_x < w / 2.0) and (dist_y < h / 2.0))

# 计算点到直线的距离
def point_line_dist(px, py, x1, y1, x2, y2):
    eps = 1e-6
    dx = x2 - x1
    dy = y2 - y1
    div = np.sqrt(dx * dx + dy * dy) + eps
    dist = np.abs(px * dy - py * dx + x2 * y1 - y2 * x1) / div
    return dist

# 将旋转框转换为多边形框
def rboxes_to_polygons(rboxes):
    """
    将旋转框转换为多边形框
    ARGS
        `rboxes`: [n, 5]
    RETURN
        `polygons`: [n, 8]
    """
    # 获取旋转框的角度
    theta = rboxes[:, 4:5]
    # 获取旋转框的中心点坐标
    cxcy = rboxes[:, :2]
    # 获取旋转框的半宽度
    half_w = rboxes[:, 2:3] / 2.
    # 获取旋转框的半高度
    half_h = rboxes[:, 3:4] / 2.
    # 计算旋转框的向量
    v1 = np.hstack([np.cos(theta) * half_w, np.sin(theta) * half_w])
    v2 = np.hstack([-np.sin(theta) * half_h, np.cos(theta) * half_h])
    # 计算多边形框的四个顶点坐标
    p1 = cxcy - v1 - v2
    p2 = cxcy + v1 - v2
    p3 = cxcy + v1 + v2
    p4 = cxcy - v1 + v2
    # 将四个顶点坐标拼接成多边形框
    polygons = np.hstack([p1, p2, p3, p4])
    return polygons

import os
import math
import random

# 默认使用阿里普惠字体
FONT_PATH = os.path.join(os.path.dirname(os.path.abspath(__file__)), "AlibabaPuHuiTi-3-45-Light.ttf")

# 将字符串转换为布尔值
def str2bool(v):
    return v.lower() in ("true", "yes", "t", "y", "1")

# 将字符串转换为整数元组
def str2int_tuple(v):
    return tuple([int(i.strip()) for i in v.split(",")])

# 绘制端到端识别结果
def draw_e2e_res(dt_boxes, strs, img_path):
    # 读取图像
    src_im = cv2.imread(img_path)
    for box, str in zip(dt_boxes, strs):
        # 将框的坐标转换为整数并调整形状
        box = box.astype(np.int32).reshape((-1, 1, 2))
        # 绘制多边形框
        cv2.polylines(src_im, [box], True, color=(255, 255, 0), thickness=2)
        # 在框上绘制文本
        cv2.putText(
            src_im,
            str,
            org=(int(box[0, 0, 0]), int(box[0, 0, 1])),
            fontFace=cv2.FONT_HERSHEY_COMPLEX,
            fontScale=0.7,
            color=(0, 255, 0),
            thickness=1,
        )
    return src_im

# 绘制文本检测结果
def draw_text_det_res(dt_boxes, img):
    for box in dt_boxes:
        # 将框的坐标转换为整数并调整形状
        box = np.array(box).astype(np.int32).reshape(-1, 2)
        # 绘制多边形框
        cv2.polylines(img, [box], True, color=(255, 255, 0), thickness=2)
    return img

# 调整图像大小
def resize_img(img, input_size=600):
    """
    调整图像大小并将图像的最长边限制为input_size
    """
    img = np.array(img)
    # 获取图像的形状
    im_shape = img.shape
    # 获取图像的最大边长
    im_size_max = np.max(im_shape[0:2])
    # 计算缩放比例
    im_scale = float(input_size) / float(im_size_max)
    # 调整图像大小
    img = cv2.resize(img, None, None, fx=im_scale, fy=im_scale)
    return img

# 绘制OCR结果
def draw_ocr_box_txt(image, boxes, txts):
    h, w = image.height, image.width
    img_left = image.copy()
    img_right = np.ones((h, w, 3), dtype=np.uint8) * 255

    import cv2
    from PIL import ImageDraw, ImageFont
    font_path = FONT_PATH
    font = ImageFont.truetype(font_path, 14)

    # 绘制左侧图像
    draw_left = ImageDraw.Draw(img_left)
    for idx, (box, txt) in enumerate(zip(boxes, txts)):
        box = np.array(box).astype(np.int32).reshape(-1, 2)
        draw_left.polylines([box], True, color=(255, 255, 0), width=2)
        draw_left.text((int(box[0, 0]), int(box[0, 1])), txt, font=font, fill=(0, 0, 255))

    # 绘制右侧图像
    draw_right = ImageDraw.Draw(img_right)
    for idx, (box, txt) in enumerate(zip(boxes, txts)):
        box = np.array(box).astype(np.int32).reshape(-1, 2)
        draw_right.polylines([box], True, color=(255, 255, 0), width=2)
        draw_right.text((int(box[0, 0]), int(box[0, 1])), txt, font=font, fill=(0, 0, 255))

    img_left = np.array(img_left)
    img_right = np.array(img_right)
    return img_left