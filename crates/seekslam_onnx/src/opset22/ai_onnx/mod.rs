#![allow(unused)]

//! 详细说明[https://github.com/onnx/onnx/blob/main/docs/Operators.md]

// 1. Absolute takes one input data (Tensor) and produces one output data (Tensor) where absolute value, y = abs(x), is applied to the tensor elementwise.
// “Absolute”操作接收一个输入数据（张量），并产生一个输出数据（张量），其中绝对值操作 \( y = \text{abs}(x) \) 会被逐元素地应用于输入张量。
pub mod op_abs;

// 2. Calculates the arccosine (inverse of cosine) of the given input tensor, element-wise.
// 逐元素计算给定输入张量的反余弦值（余弦的逆函数）。
pub mod op_acos;

// 3. Calculates the hyperbolic arccosine of the given input tensor element-wise.
// 计算给定输入张量的双曲反余弦值，逐元素进行计算。
pub mod op_acosh;

// 4. Performs element-wise binary addition (with Numpy-style broadcasting support).
// 执行逐元素的二元加法运算（支持类似 NumPy 的广播机制）。
pub mod op_add;

// 5. Returns the tensor resulted from performing the and logical operation elementwise on the input tensors A and B (with Numpy-style broadcasting support).
// 返回对输入张量 A 和 B 进行逐元素的逻辑与操作后得到的张量（支持类似 NumPy 的广播机制）。
pub mod op_and;

// 6. Computes the indices of the max elements of the input tensor's element along the provided axis. 
// 计算输入张量在指定轴上元素的最大值的索引。
pub mod op_argmax;

// 7. Computes the indices of the min elements of the input tensor's element along the provided axis.
// 计算输入张量在指定轴上元素的最小值的索引。
pub mod op_argmin;

// 8. Calculates the arcsine (inverse of sine) of the given input tensor, element-wise.
// 逐元素计算给定输入张量的反正弦值（正弦的逆运算）。
pub mod op_asin;

// 9. Calculates the hyperbolic arcsine of the given input tensor element-wise.
// 逐元素计算给定输入张量的反双曲正弦值。
pub mod op_asinh;

// 10. Arctangent 算子
// 逐元素计算输入张量的反正切值
// 数学表达式: y = atan(x)
pub mod op_atan;

// 11. Hyperbolic Arctangent 算子
// 逐元素计算输入张量的反双曲正切值
// 数学表达式: y = atanh(x)
pub mod op_atanh;

// 12. Average Pooling 算子
// 执行平均池化操作
pub mod op_average_pool;

// 13. Batch Normalization 算子
// 执行批量归一化操作
pub mod op_batch_normalization;

// 14. Bit Shift 算子
// 执行位移动操作
pub mod op_bit_shift;

// 15. Bitwise AND 算子
// 逐元素执行两个输入张量的位与运算
pub mod op_bitwise_and;

// 16. Bitwise NOT 算子
// 逐元素执行输入张量的位非运算
pub mod op_bitwise_not;

// 17. Bitwise OR 算子
// 逐元素执行两个输入张量的位或运算
pub mod op_bitwise_or;

// 18. Bitwise XOR 算子
// 逐元素执行两个输入张量的位异或运算
pub mod op_bitwise_xor;

// 19. Cast 算子
// 将输入张量转换为指定数据类型
pub mod op_cast;

// 20. Ceiling 算子
// 逐元素向上取整
// 数学表达式: y = ceil(x)
pub mod op_ceil;

// 21. Col2Im 算子
// 将列矩阵转换为图像张量
pub mod op_col2im;

// 22. Compress 算子
// 根据条件选择输入张量的元素
pub mod op_compress;

// 23. Concat 算子
// 沿指定轴连接多个输入张量
pub mod op_concat;

// 24. ConcatFromSequence 算子
// 将序列中的张量沿指定轴连接
pub mod op_concat_from_sequence;

// 25. Constant 算子
// 生成常量张量
pub mod op_constant;

// 26. ConstantOfShape 算子
// 根据输入形状生成常量张量
pub mod op_constant_of_shape;

// 27. Convolution 算子
// 执行卷积操作
pub mod op_conv;

// 28. Integer Convolution 算子
// 执行整数卷积操作
pub mod op_conv_integer;

// 29. Transposed Convolution 算子
// 执行转置卷积操作
pub mod op_conv_transpose;

// 30. Cosine 算子
// 逐元素计算余弦值
// 数学表达式: y = cos(x)
pub mod op_cos;

// 31. Hyperbolic Cosine 算子
// 逐元素计算双曲余弦值
// 数学表达式: y = cosh(x)
pub mod op_cosh;

// 32. Cumulative Sum 算子
// 计算输入张量的累积和
pub mod op_cum_sum;

// 33. Discrete Fourier Transform 算子
// 执行离散傅里叶变换
pub mod op_dft;

// 34. Deformable Convolution 算子
// 执行可变形卷积操作
pub mod op_deform_conv;

// 35. DepthToSpace 算子
// 将深度维度数据重新排列为空间维度
pub mod op_depth_to_space;

// 36. Dequantize Linear 算子
// 执行线性反量化操作
pub mod op_dequantize_linear;

// 37. Determinant 算子
// 计算方阵的行列式
pub mod op_det;

// 38. Division 算子
// 逐元素执行两个输入张量的除法运算，支持广播
pub mod op_div;

// 39. Dropout 算子
// 执行dropout操作，随机将输入元素置零
pub mod op_dropout;

// 40. Einsum 算子
// 执行爱因斯坦求和约定操作
pub mod op_einsum;

// 41. Equal 算子
// 逐元素比较两个输入张量是否相等，支持广播
pub mod op_equal;

// 42. Error Function 算子
// 逐元素计算误差函数
// 数学表达式: y = erf(x)
pub mod op_erf;

// 43. Exponential 算子
// 逐元素计算指数值
// 数学表达式: y = exp(x)
pub mod op_exp;

// 44. Expand 算子
// 将输入张量扩展到指定形状，支持广播
pub mod op_expand;

// 45. EyeLike 算子
// 生成类似单位矩阵的张量
pub mod op_eyelike;

// 46. Flatten 算子
// 将输入张量展平为一维或指定形状
pub mod op_flatten;

// 47. Floor 算子
// 逐元素向下取整
// 数学表达式: y = floor(x)
pub mod op_floor;

// 48. GRU 算子
// 执行门控循环单元(Gated Recurrent Unit)操作
pub mod op_gru;

// 49. Gather 算子
// 根据索引从输入张量收集元素
pub mod op_gather;

// 50. GatherElements 算子
// 根据元素级索引从输入张量收集元素
pub mod op_gather_elements;

// 51. GatherND 算子
// 根据N维索引从输入张量收集元素
pub mod op_gather_nd;

// 52. General Matrix Multiplication 算子
// 执行通用矩阵乘法操作
pub mod op_gemm;

// 53. Global Average Pooling 算子
// 执行全局平均池化操作
pub mod op_global_average_pool;

// 54. Global Lp Pooling 算子
// 执行全局Lp范数池化操作
pub mod op_global_lp_pool;

// 55. Global Max Pooling 算子
// 执行全局最大池化操作
pub mod op_global_max_pool;

// 56. Greater 算子
// 逐元素比较第一个输入是否大于第二个输入，支持广播
pub mod op_greater;

// 57. GridSample 算子
// 根据网格对输入张量进行采样
pub mod op_grid_sample;

// 58. Hardmax 算子
// 执行硬最大函数操作
pub mod op_hardmax;

// 59. Identity 算子
// 返回与输入相同的输出
pub mod op_identity;

// 60. If 算子
// 条件执行算子，根据条件选择执行路径
pub mod op_if;

// 61. Image Decoder 算子
// 图像解码器算子
pub mod op_image_decoder;

// 62. Instance Normalization 算子
// 执行实例归一化操作
pub mod op_instance_normalization;

// 63. IsInf 算子
// 检测输入张量元素是否为无穷大
pub mod op_is_inf;

// 64. IsNaN 算子
// 检测输入张量元素是否为NaN
pub mod op_is_nan;

// 65. Local Response Normalization 算子
// 执行局部响应归一化操作
pub mod op_lrn;

// 66. LSTM 算子
// 执行长短期记忆网络(Long Short-Term Memory)操作
pub mod op_lstm;

// 67. Less 算子
// 逐元素比较第一个输入是否小于第二个输入，支持广播
pub mod op_less;

// 68. Logarithm 算子
// 逐元素计算自然对数
// 数学表达式: y = log(x)
pub mod op_log;

// 69. Loop 算子
// 循环执行子图
pub mod op_loop;

// 70. Lp Normalization 算子
// 执行Lp范数归一化操作
pub mod op_lp_normalization;

// 71. Lp Pooling 算子
// 执行Lp范数池化操作
pub mod op_lp_pool;

// 72. Matrix Multiplication 算子
// 执行矩阵乘法操作
pub mod op_mat_mul;

// 73. Integer Matrix Multiplication 算子
// 执行整数矩阵乘法操作
pub mod op_mat_mul_integer;

// 74. Maximum 算子
// 逐元素计算多个输入张量的最大值，支持广播
pub mod op_max;

// 75. Max Pooling 算子
// 执行最大池化操作
pub mod op_max_pool;

// 76. Max ROI Pooling 算子
// 执行基于感兴趣区域的最大池化操作
pub mod op_max_roi_pool;

// 77. Max Unpooling 算子
// 执行最大反池化操作
pub mod op_max_unpool;

// 78. Mean 算子
// 逐元素计算多个输入张量的平均值，支持广播
pub mod op_mean;

// 79. Mel Weight Matrix 算子
// 生成梅尔滤波器组权重矩阵
pub mod op_mel_weight_matrix;

// 80. Minimum 算子
// 逐元素计算多个输入张量的最小值，支持广播
pub mod op_min;

// 81. Modulo 算子
// 逐元素执行模运算
pub mod op_mod;

// 82. Multiplication 算子
// 逐元素执行两个输入张量的乘法运算，支持广播
pub mod op_mul;

// 83. Multinomial 算子
// 从多项分布中采样
pub mod op_multinomial;

// 84. Negative 算子
// 逐元素取负
// 数学表达式: y = -x
pub mod op_neg;

// 85. Non-Maximum Suppression 算子
// 执行非极大值抑制操作
pub mod op_non_max_suppression;

// 86. Non-Zero 算子
// 返回输入张量中非零元素的索引
pub mod op_non_zero;

// 87. Logical NOT 算子
// 逐元素执行逻辑非运算
pub mod op_not;

// 88. One-Hot 算子
// 将索引转换为one-hot编码
pub mod op_onehot;

// 89. Optional 算子
// 处理可选类型数据
pub mod op_optional;

// 90. Optional Get Element 算子
// 从可选类型中获取元素
pub mod op_optional_get_element;

// 91. Optional Has Element 算子
// 检查可选类型是否包含元素
pub mod op_optional_has_element;

// 92. Logical OR 算子
// 逐元素执行两个输入张量的逻辑或运算，支持广播
pub mod op_or;

// 93. Pad 算子
// 对输入张量进行填充
pub mod op_pad;

// 94. Power 算子
// 逐元素执行幂运算
// 数学表达式: y = x^exponent
pub mod op_pow;

// 95. Quantized Linear Convolution 算子
// 执行量化线性卷积操作
pub mod op_q_linear_conv;

// 96. Quantized Linear Matrix Multiplication 算子
// 执行量化线性矩阵乘法操作
pub mod op_q_linear_mat_mul;

// 97. Quantize Linear 算子
// 执行线性量化操作
pub mod op_quantize_linear;

// 98. RNN 算子
// 执行循环神经网络操作
pub mod op_rnn;

// 99. Random Normal 算子
// 从正态分布生成随机数
pub mod op_random_normal;

// 100. Random Normal Like 算子
// 从正态分布生成与输入形状相同的随机数
pub mod op_random_normal_like;

// 101. Random Uniform 算子
// 从均匀分布生成随机数
pub mod op_random_uniform;

// 102. Random Uniform Like 算子
// 从均匀分布生成与输入形状相同的随机数
pub mod op_random_uniform_like;

// 103. Reciprocal 算子
// 逐元素计算倒数
// 数学表达式: y = 1/x
pub mod op_reciprocal;

// 104. Reduce Max 算子
// 沿指定轴计算输入张量的最大值
pub mod op_reduce_max;

// 105. Reduce Mean 算子
// 沿指定轴计算输入张量的平均值
pub mod op_reduce_mean;

// 106. Reduce Min 算子
// 沿指定轴计算输入张量的最小值
pub mod op_reduce_min;

// 107. Reduce Product 算子
// 沿指定轴计算输入张量的乘积
pub mod op_reduce_prod;

// 108. Reduce Sum 算子
// 沿指定轴计算输入张量的和
pub mod op_reduce_sum;

// 109. Regex Full Match 算子
// 执行正则表达式完全匹配
pub mod op_regex_full_match;

// 110. Reshape 算子
// 改变输入张量的形状
pub mod op_reshape;

// 111. Resize 算子
// 调整输入张量的空间尺寸
pub mod op_resize;

// 112. Reverse Sequence 算子
// 反转输入张量的序列
pub mod op_reverse_sequence;

// 113. ROI Align 算子
// 执行感兴趣区域对齐操作
pub mod op_roi_align;

// 114. Round 算子
// 逐元素四舍五入
pub mod op_round;

// 115. Short-Time Fourier Transform 算子
// 执行短时傅里叶变换
pub mod op_stft;

// 116. Scan 算子
// 扫描算子，沿指定轴迭代执行子图
pub mod op_scan;

// 117. Scatter 算子(废弃⚠️)
// 根据索引更新张量
pub mod _op_scatter;

// 118. Scatter Elements 算子
// 根据元素级索引更新张量
pub mod op_scatter_elements;

// 119. Scatter ND 算子
// 根据N维索引更新张量
pub mod op_scatter_nd;

// 120. Sequence At 算子
// 获取序列中指定位置的元素
pub mod op_sequence_at;

// 121. Sequence Construct 算子
// 从多个张量构造序列
pub mod op_sequence_construct;

// 122. Sequence Empty 算子
// 创建空序列
pub mod op_sequence_empty;

// 123. Sequence Erase 算子
// 从序列中删除元素
pub mod op_sequence_erase;

// 124. Sequence Insert 算子
// 向序列中插入元素
pub mod op_sequence_insert;

// 125. Sequence Length 算子
// 获取序列长度
pub mod op_sequence_length;

// 126. Shape 算子
// 获取输入张量的形状
pub mod op_shape;

// 127. Sigmoid 算子
// 逐元素计算sigmoid函数
// 数学表达式: y = 1 / (1 + exp(-x))
pub mod op_sigmoid;

// 128. Sign 算子
// 逐元素获取符号
// 数学表达式: y = sign(x)
pub mod op_sign;

// 129. Sine 算子
// 逐元素计算正弦值
// 数学表达式: y = sin(x)
pub mod op_sin;

// 130. Hyperbolic Sine 算子
// 逐元素计算双曲正弦值
// 数学表达式: y = sinh(x)
pub mod op_sinh;

// 131. Size 算子
// 获取输入张量的元素总数
pub mod op_size;

// 132. Slice 算子
// 从输入张量中提取切片
pub mod op_slice;

// 133. SpaceToDepth 算子
// 将空间维度数据重新排列为深度维度
pub mod op_space_to_depth;

// 134. Split 算子
// 沿指定轴分割输入张量
pub mod op_split;

// 135. Split To Sequence 算子
// 将输入张量分割为序列
pub mod op_split_to_sequence;

// 136. Square Root 算子
// 逐元素计算平方根
// 数学表达式: y = sqrt(x)
pub mod op_sqrt;

// 137. Squeeze 算子
// 移除输入张量中长度为1的维度
pub mod op_squeeze;

// 138. String Concatenation 算子
// 字符串连接操作
pub mod op_string_concat;

// 139. String Normalizer 算子
// 字符串规范化操作
pub mod op_string_normalizer;

// 140. String Split 算子
// 字符串分割操作
pub mod op_string_split;

// 141. Subtraction 算子
// 逐元素执行两个输入张量的减法运算，支持广播
pub mod op_sub;

// 142. Sum 算子
// 逐元素计算多个输入张量的和，支持广播
pub mod op_sum;

// 143. Tangent 算子
// 逐元素计算正切值
// 数学表达式: y = tan(x)
pub mod op_tan;

// 144. Hyperbolic Tangent 算子
// 逐元素计算双曲正切值
// 数学表达式: y = tanh(x)
pub mod op_tanh;

// 145. TF-IDF Vectorizer 算子
// 执行TF-IDF向量化操作
pub mod op_tf_idf_vectorizer;

// 146. Tile 算子
// 沿指定维度复制输入张量
pub mod op_tile;

// 147. TopK 算子
// 获取输入张量沿指定轴的前K个最大/最小值
pub mod op_topk;

// 148. Transpose 算子
// 转置输入张量的维度
pub mod op_transpose;

// 149. Triangular 算子
// 执行三角矩阵操作
pub mod op_trilu;

// 150. Unique 算子
// 查找输入张量中的唯一元素
pub mod op_unique;

// 151. Unsqueeze 算子
// 在指定位置插入长度为1的维度
pub mod op_unsqueeze;

// 152. Upsample 算子(废弃⚠️)
// 上采样操作
pub mod _op_upsample;

// 153. Where 算子
// 根据条件选择元素
pub mod op_where;

// 154. Logical XOR 算子
// 逐元素执行两个输入张量的逻辑异或运算，支持广播
pub mod op_xor;

// 155. Function 算子
// 自定义函数算子
pub mod op_function;

// 156. Affine Grid 算子
// 生成仿射变换网格
pub mod fn_affine_grid;

// 157. Attention 算子
// 执行注意力机制操作
pub mod fn_attention;

// 158. Bernoulli 算子
// 从伯努利分布中采样
pub mod fn_bernoulli;

// 159. Blackman Window 算子
// 生成布莱克曼窗函数
pub mod fn_blackman_window;

// 160. Cast Like 算子
// 将输入转换为与参考张量相同的数据类型
pub mod fn_cast_like;

// 161. CELU 算子
// 执行连续指数线性单元激活函数
pub mod fn_celu;

// 162. Center Crop Pad 算子
// 执行中心裁剪和填充操作
pub mod fn_center_crop_pad;

// 163. Clip 算子
// 将输入值限制在指定范围内
pub mod fn_clip;

// 164. Dynamic Quantize Linear 算子
// 执行动态线性量化操作
pub mod fn_dynamic_quantize_linear;

// 165. ELU 算子
// 执行指数线性单元激活函数
pub mod fn_elu;

// 166. GELU 算子
// 执行高斯误差线性单元激活函数
pub mod fn_gelu;

// 167. Greater Or Equal 算子
// 逐元素比较第一个输入是否大于等于第二个输入，支持广播
pub mod fn_greater_or_equal;

// 168. Group Normalization 算子
// 执行组归一化操作
pub mod fn_group_normalization;

// 169. Hamming Window 算子
// 生成汉明窗函数
pub mod fn_hamming_window;

// 170. Hann Window 算子
// 生成汉宁窗函数
pub mod fn_hann_window;

// 171. Hard Sigmoid 算子
// 执行硬sigmoid激活函数
pub mod fn_hard_sigmoid;

// 172. Hard Swish 算子
// 执行硬swish激活函数
pub mod fn_hard_swish;

// 173. Layer Normalization 算子
// 执行层归一化操作
pub mod fn_layer_normalization;

// 174. Leaky ReLU 算子
// 执行泄漏修正线性单元激活函数
pub mod fn_leaky_relu;

// 175. Less Or Equal 算子
// 逐元素比较第一个输入是否小于等于第二个输入，支持广播
pub mod fn_less_or_equal;

// 176. Log Softmax 算子
// 执行对数softmax操作
pub mod fn_log_softmax;

// 177. Mean Variance Normalization 算子
// 执行均值方差归一化操作
pub mod fn_mean_variance_normalization;

// 178. Mish 算子
// 执行Mish激活函数
pub mod fn_mish;

// 179. Negative Log Likelihood Loss 算子
// 计算负对数似然损失
pub mod fn_negative_log_likelihood_loss;

// 180. Parametric ReLU 算子
// 执行参数化修正线性单元激活函数
pub mod fn_p_relu;

// 181. RMS Normalization 算子
// 执行均方根归一化操作
pub mod fn_rms_normalization;

// 182. Range 算子
// 生成指定范围内的数值序列
pub mod fn_range;

// 183. Reduce L1 算子
// 沿指定轴计算输入张量的L1范数
pub mod fn_reduce_l1;

// 184. Reduce L2 算子
// 沿指定轴计算输入张量的L2范数
pub mod fn_reduce_l2;

// 185. Reduce Log Sum 算子
// 沿指定轴计算输入张量的对数和
pub mod fn_reduce_log_sum;

// 186. Reduce Log Sum Exp 算子
// 沿指定轴计算输入张量的对数求和指数
pub mod fn_reduce_log_sum_exp;

// 187. Reduce Sum Square 算子
// 沿指定轴计算输入张量的平方和
pub mod fn_reduce_sum_square;

// 188. ReLU 算子
// 执行修正线性单元激活函数
pub mod fn_relu;

// 189. Rotary Embedding 算子
// 执行旋转位置嵌入操作
pub mod fn_rotary_embedding;

// 190. SELU 算子
// 执行缩放指数线性单元激活函数
pub mod fn_selu;

// 191. Sequence Map 算子
// 对序列中的每个元素应用子图
pub mod fn_sequence_map;

// 192. Shrink 算子
// 执行收缩操作
pub mod fn_shrink;

// 193. Softmax 算子
// 执行softmax操作
pub mod fn_softmax;

// 194. Softmax Cross Entropy Loss 算子
// 计算softmax交叉熵损失
pub mod fn_softmax_cross_entropy_loss;

// 195. Softplus 算子
// 执行softplus激活函数
pub mod fn_softplus;

// 196. Softsign 算子
// 执行softsign激活函数
pub mod fn_softsign;

// 197. Thresholded ReLU 算子
// 执行阈值修正线性单元激活函数
pub mod fn_thresholded_relu;
