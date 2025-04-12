# SAM 2.1 图像分割数学原理

## 示例
```py
# 测试SAM2.1图片分割

# 编码模型: ../../../assets/ailia-models/segment-anything-2/sam2.1_base_plus.encoder.onnx
## 输出: 
## high_res_feats_0
## name: high_res_feats_0
## tensor: float32[Reshapehigh_res_feats_0_dim_0,Reshapehigh_res_feats_0_dim_1,Reshapehigh_res_feats_0_dim_2,Reshapehigh_res_feats_0_dim_3]
## high_res_feats_1
## name: high_res_feats_1
## tensor: float32[Reshapehigh_res_feats_1_dim_0,Reshapehigh_res_feats_1_dim_1,Reshapehigh_res_feats_1_dim_2,Reshapehigh_res_feats_1_dim_3]
## image_embed
## name: image_embed
## tensor: float32[Reshapeimage_embed_dim_0,Reshapeimage_embed_dim_1,Reshapeimage_embed_dim_2,Reshapeimage_embed_dim_3]

# 解码模型: ../../../assets/ailia-models/segment-anything-2/sam2.1_base_plus.decoder.onnx
## 输出:
## masks
## name: masks
## tensor: float32[Resizemasks_dim_0,Resizemasks_dim_1,Resizemasks_dim_2,Resizemasks_dim_3]
## iou_predictions
## name: iou_predictions
## tensor: float32[Resizemasks_dim_0,Whereiou_predictions_dim_1]

# 输入图片../assets/rgb1.png
# 输出../result/segment_onnx.png

import os
import cv2
import numpy as np
import onnxruntime as ort

class SAM2ImagePredictor:
    def __init__(self, encoder_model_path, decoder_model_path):
        """
        初始化 SAM2 图像分割预测器
        :param encoder_model_path: 编码器模型路径
        :param decoder_model_path: 解码器模型路径
        """
        self.encoder_session = ort.InferenceSession(encoder_model_path)
        self.decoder_session = ort.InferenceSession(decoder_model_path)

    def trunc_normal(self, size, std=0.02, a=-2, b=2):
        """
        生成截断正态分布的随机数
        :param size: 输出数组的形状
        :param std: 标准差
        :param a: 截断下限
        :param b: 截断上限
        :return: 截断正态分布的随机数数组
        """
        values = np.random.normal(loc=0., scale=std, size=size)
        values = np.clip(values, a * std, b * std)
        return values

    def set_image(self, image):
        """
        设置输入图像并获取编码器的特征
        :param image: 输入图像
        :return: 编码器的特征
        """
        # 调整图像大小为模型期望的尺寸
        image = cv2.resize(image, (1024, 1024))
        # 调整通道顺序为模型期望的顺序（CHW）
        image = np.transpose(image, (2, 0, 1))
        # 添加批量维度
        image = np.expand_dims(image, axis=0).astype(np.float32)
        outputs = self.encoder_session.run(None, {"image": image})
        high_res_feats_0, high_res_feats_1, image_embed = outputs
        features = {
            "high_res_feats_0": high_res_feats_0,
            "high_res_feats_1": high_res_feats_1,
            "image_embed": image_embed
        }
        return features

    def predict(self, features, orig_hw, point_coords=None, point_labels=None, box=None, mask_input=None):
        """
        进行图像分割预测
        :param features: 编码器的特征
        :param orig_hw: 原始图像的高和宽
        :param point_coords: 点提示的坐标
        :param point_labels: 点提示的标签
        :param box: 边框提示
        :param mask_input: 掩码输入
        :return: 分割掩码、IoU 预测值和低分辨率掩码
        """
        if point_coords is not None and len(point_coords) != 0:
            point_coords = point_coords.astype(np.float32)
            unnorm_coords = self.transform_coords(point_coords, orig_hw)
            labels = point_labels.astype(np.float32)
            if len(unnorm_coords.shape) == 2:
                unnorm_coords, labels = unnorm_coords[None, ...], labels[None, ...]
        else:
            unnorm_coords, labels = None, None

        if box is not None:
            box = box.astype(np.float32)
            unnorm_box = self.transform_boxes(box, orig_hw)
        else:
            unnorm_box = None

        if mask_input is not None:
            mask_input = mask_input.astype(np.float32)
            if len(mask_input.shape) == 3:
                mask_input = mask_input[None, :, :, :]
        else:
            # 确保 mask_input 的维度为 4
            mask_input = np.zeros((1, 1, 256, 256), dtype=np.float32)

        # 如果没有点提示和边框提示，创建一个默认的点提示
        if unnorm_coords is None and unnorm_box is None:
            unnorm_coords = np.array([[[0.5, 0.5]]], dtype=np.float32)  # 默认点提示在图像中心
            labels = np.array([[1]], dtype=np.float32)  # 默认标签为1，修改为 float32

        if unnorm_coords is not None:
            concat_points = (unnorm_coords, labels)
        else:
            concat_points = None

        if unnorm_box is not None:
            box_coords = unnorm_box.reshape(-1, 2, 2)
            box_labels = np.array([[2, 3]], dtype=np.float32)  # 修改为 float32
            box_labels = box_labels.repeat(unnorm_box.shape[0], 1)
            if concat_points is not None:
                concat_coords = np.concatenate([box_coords, concat_points[0]], axis=1)
                concat_labels = np.concatenate([box_labels, concat_points[1]], axis=1)
                concat_points = (concat_coords, concat_labels.astype(np.int32))
            else:
                concat_points = (box_coords, box_labels.astype(np.int32))

        if mask_input is None:
            mask_input_dummy = np.zeros((1, 256, 256), dtype=np.float32)
            masks_enable = np.array([0], dtype=np.float32)
        else:
            mask_input_dummy = mask_input
            masks_enable = np.array([1], dtype=np.float32)

        if concat_points is None:
            raise ValueError("concat_points must be exists")

        orig_im_size = np.array(orig_hw, dtype=np.int32)

        sparse_embeddings, dense_embeddings = self.decoder_session.run(
            None,
            {
                "point_coords": concat_points[0],  # 修改为模型期望的输入名称 f32
                "point_labels": concat_points[1],  # 修改为模型期望的输入名称 f32
                "mask_input": mask_input_dummy,   # 修改为模型期望的输入名称 f32
                "has_mask_input": masks_enable,   # 修改为模型期望的输入名称 f32
                "orig_im_size": orig_im_size,     # 添加模型期望的输入 i32
                "image_embed": features["image_embed"],  # 修改为模型期望的输入名称 f32
                "high_res_feats_0": features["high_res_feats_0"],  # 修改为模型期望的输入名称 f32
                "high_res_feats_1": features["high_res_feats_1"]   # 修改为模型期望的输入名称 f32
            }
        )

        masks, iou_pred = sparse_embeddings, dense_embeddings

        low_res_masks = masks[:, 1:, :, :]
        iou_predictions = iou_pred[:, 1:]

        masks = self.postprocess_masks(masks, orig_hw)

        return masks, iou_predictions, low_res_masks

    def transform_coords(self, coords, orig_hw):
        """
        转换坐标
        :param coords: 坐标
        :param orig_hw: 原始图像的高和宽
        :return: 转换后的坐标
        """
        h, w = orig_hw
        coords = coords.copy()
        coords[..., 0] = coords[..., 0] / w
        coords[..., 1] = coords[..., 1] / h

        resolution = 1024
        coords = coords * resolution
        return coords

    def transform_boxes(self, boxes, orig_hw):
        """
        转换边框
        :param boxes: 边框
        :param orig_hw: 原始图像的高和宽
        :return: 转换后的边框
        """
        boxes = self.transform_coords(boxes.reshape(-1, 2, 2), orig_hw)
        return boxes

    def postprocess_masks(self, masks, orig_hw):
        interpolated_masks = []
        for mask in masks:
            # 打印 mask 的形状，便于调试
            print("Original mask shape:", mask.shape)

            # 确保 mask 的形状为 (num_masks, height, width)
            if len(mask.shape) == 4:  # 如果有 batch 维度，去掉 batch 维度
                mask = np.squeeze(mask, axis=0)
            if len(mask.shape) == 3:  # 如果已经是 (num_masks, height, width)，直接使用
                pass
            else:
                raise ValueError(f"Unexpected mask shape: {mask.shape}")

            # 转换为 (height, width, num_masks)
            mask = np.transpose(mask, (1, 2, 0))

            # 调整大小到原始图像尺寸
            resized_mask = cv2.resize(mask, (orig_hw[1], orig_hw[0]), interpolation=cv2.INTER_LINEAR)

            # 打印 resized_mask 的形状，便于调试
            print("Resized mask shape:", resized_mask.shape)

            # 如果 resized_mask 是二维的，添加一个新的维度
            if len(resized_mask.shape) == 2:
                resized_mask = resized_mask[:, :, np.newaxis]

            # 转换回 (num_masks, height, width)
            resized_mask = np.transpose(resized_mask, (2, 0, 1))

            interpolated_masks.append(resized_mask)

        interpolated_masks = np.array(interpolated_masks)

        return interpolated_masks

if __name__ == "__main__":
    # 输入图片路径
    input_image_path = "../assets/rgb1.png"
    # 输出图片路径
    output_image_path = "../result/segment_onnx.png"
    # 编码器模型路径
    encoder_model_path = "../../../assets/ailia-models/segment-anything-2/sam2.1_base_plus.encoder.onnx"
    # 解码器模型路径
    decoder_model_path = "../../../assets/ailia-models/segment-anything-2/sam2.1_base_plus.decoder.onnx"

    # 读取输入图片
    image = cv2.imread(input_image_path)
    image = cv2.cvtColor(image, cv2.COLOR_BGR2RGB)
    orig_hw = image.shape[:2]

    # 初始化 SAM2 图像分割预测器
    predictor = SAM2ImagePredictor(encoder_model_path, decoder_model_path)

    # 设置输入图像并获取编码器的特征
    features = predictor.set_image(image)

    # 进行图像分割预测
    masks, iou_predictions, low_res_masks = predictor.predict(features, orig_hw)

    # 将分割掩码转换为可视化图像
    mask = masks[0, 0].astype(np.uint8) * 255
    mask = cv2.cvtColor(mask, cv2.COLOR_GRAY2BGR)

    # 将分割掩码叠加到原始图像上
    result = cv2.addWeighted(image, 0.5, mask, 0.5, 0)
    result = cv2.cvtColor(result, cv2.COLOR_RGB2BGR)

    # 保存输出图片
    cv2.imwrite(output_image_path, result)
```

## 原理
以下是代码中涉及的数学原理，使用 Markdown 和 KaTeX 表示：

### 截断正态分布的随机数生成
在 `trunc_normal` 函数中，生成截断正态分布的随机数，其数学表达式为：
\[ \text{values} = \text{np.random.normal(loc=0., scale=std, size=size)} \]
然后对生成的值进行截断：
\[ \text{values} = \text{np.clip(values, a * std, b * std)} \]
其中，`std` 是标准差，`a` 和 `b` 是截断的上下限。

### 坐标转换
在 `transform_coords` 函数中，将坐标从原始图像尺度转换到模型期望的尺度，其数学表达式为：
\[ \text{coords[..., 0]} = \frac{\text{coords[..., 0]}}{w} \]
\[ \text{coords[..., 1]} = \frac{\text{coords[..., 1]}}{h} \]
然后将坐标缩放到模型的分辨率（假设为 1024）：
\[ \text{coords} = \text{coords} \times \text{resolution} \]
其中，`h` 和 `w` 分别是原始图像的高和宽。

### 边框转换
在 `transform_boxes` 函数中，将边框从原始图像尺度转换到模型期望的尺度，其数学表达式为：
\[ \text{boxes} = \text{transform_coords(boxes.reshape(-1, 2, 2), orig_hw)} \]
即将边框的四个顶点坐标分别进行坐标转换。

### 掩码后处理
在 `postprocess_masks` 函数中，对分割掩码进行后处理，主要包括调整大小和维度转换。调整大小的数学表达式为：
\[ \text{resized_mask} = \text{cv2.resize(mask, (orig_hw[1], orig_hw[0]), interpolation=cv2.INTER_LINEAR)} \]
然后将掩码的维度从 `(num_masks, height, width)` 转换为 `(height, width, num_masks)`，再转回 `(num_masks, height, width)`。

### 掩码解码器的输入
在 `predict` 函数中，将各种输入传递给掩码解码器，其输入包括：
- 图像嵌入：`image_embed`
- 高分辨率特征：`high_res_feats_0` 和 `high_res_feats_1`
- 点提示坐标和标签：`point_coords` 和 `point_labels`
- 边框提示：`box`
- 掩码输入：`mask_input`
- 原始图像尺寸：`orig_im_size`
这些输入被传递给解码器，生成分割掩码和 IoU 预测值。

### 分割掩码的生成
在 `predict` 函数中，解码器输出的分割掩码经过后处理，最终得到的分割掩码的数学表达式为：
\[ \text{masks} = \text{postprocess_masks(masks, orig_hw)} \]
即对解码器输出的掩码进行后处理，调整到原始图像的尺寸。