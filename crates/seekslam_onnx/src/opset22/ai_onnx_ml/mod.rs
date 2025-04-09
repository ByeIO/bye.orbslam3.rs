#![allow(unused)]

//! 详细说明[https://github.com/onnx/onnx/blob/main/docs/Operators.md]

// 1. Select elements of the input tensor based on the indices passed.The indices are applied to the last axes of the tensor.
// 根据传入的索引选择输入张量的元素。这些索引应用于张量的最后一个轴。
pub mod op_array_feature_extractor;

// 2. Maps the values of the input tensor to either 0 or 1, element-wise, based on the outcome of a comparison against a threshold value.
// 将输入张量的值逐元素地映射为0或1，具体取决于与阈值的比较结果。
pub mod op_binarizer;

// 3. Converts a map to a tensor.The map key must be an int64 and the values will be ordered in ascending order based on this key.The operator supports dense packing or sparse packing. If using sparse packing, the key cannot exceed the max_map-1 value.
// 将一个映射（map）转换为张量。映射的键必须是`int64`类型，而值将根据这个键按升序排列。该操作符支持密集打包（dense packing）或稀疏打包（sparse packing）。如果使用稀疏打包，键的值不能超过`max_map-1`。
pub mod op_cast_map;

// 4. Converts strings to integers and vice versa.Two sequences of equal length are used to map between integers and strings, with strings and integers at the same index detailing the mapping.Each operator converts either integers to strings or strings to integers, depending on which default value attribute is provided. Only one default value attribute should be defined.If the string default value is set, it will convert integers to strings. If the int default value is set, it will convert strings to integers.
// 将字符串转换为整数，反之亦然。使用两个等长的序列来映射整数和字符串之间的关系，相同索引位置的字符串和整数表示它们之间的映射关系。每个操作符根据提供的默认值属性，将整数转换为字符串或字符串转换为整数。只能定义一个默认值属性。如果设置了字符串默认值，它将把整数转换为字符串。如果设置了整数默认值，它将把字符串转换为整数。
pub mod op_category_mapper;

// 5. Uses an index mapping to convert a dictionary to an array.
// 使用索引映射将字典转换为数组。
pub mod op_dict_vectorizer;

// 6. Concatenates input tensors into one continuous output.
// 将输入的张量拼接成一个连续的输出。
pub mod op_feature_vectorizer;

// 7. Replaces inputs that equal one value with another, leaving all other elements alone.
// 将输入中等于某个特定值的元素替换为另一个值，而保持其他所有元素不变。
pub mod op_imputer;

// 8. Maps each element in the input tensor to another value.
// 将输入张量中的每个元素映射到另一个值。
pub mod op_label_encoder;

// 9. Linear classifier
// 线性分类器
pub mod op_linear_classifier;

// 10. Generalized linear regression evaluation.
// 广义线性回归评估。
pub mod op_linear_regressor;

// 11. Normalize the input.
// 对输入进行归一化。
pub mod op_normalizer;

// 12. Replace each input element with an array of ones and zeros, where a single one is placed at the index of the category that was passed in.
// 将每个输入元素替换为一个由1和0组成的数组，其中在传入类别的索引位置放置一个单一的1。
pub mod op_onehot_encoder;

// 13. Support Vector Machine classifier
// 支持向量机分类器
pub mod op_svm_classfier;

// 14. Support Vector Machine regression prediction and one-class SVM anomaly detection.
// 支持向量机回归预测以及单类支持向量机异常检测。
pub mod op_svm_regressor;

// 15. Rescale input data, for example to standardize features by removing the mean and scaling to unit variance.
// 重新缩放输入数据，例如通过去除均值并缩放到单位方差来标准化特征。
pub mod op_scaler;

// 16. Tree Ensemble operator. 
// 树集成操作符。
pub mod op_tree_ensemble;

// 17. This operator is DEPRECATED. Please use TreeEnsemble with provides similar functionality. In order to determine the top class, the ArgMax node can be applied to the output of TreeEnsemble. To encode class labels, use a LabelEncoder operator. Tree Ensemble classifier. Returns the top class for each of N inputs.
// 这个操作符已被弃用⚠️。请使用提供类似功能的`TreeEnsemble`。为了确定最高类别，可以将`ArgMax`节点应用于`TreeEnsemble`的输出。为了对类别标签进行编码，请使用`LabelEncoder`操作符。树集成分类器。为每个N个输入返回最高类别。
pub mod _op_tree_ensemble_classifier;

// 18. This operator is DEPRECATED. Please use TreeEnsemble instead which provides the same functionality.Tree Ensemble regressor. Returns the regressed values for each input in N.
// 这个操作符已被弃用⚠️。请改用 TreeEnsemble，它提供了相同的功能。树集成回归器。为输入中的每个值返回回归值。
pub mod _op_tree_ensemble_regressor;

// 19. Creates a map from the input and the attributes.
// 从输入和属性中创建一个映射。
pub mod op_zip_map;
