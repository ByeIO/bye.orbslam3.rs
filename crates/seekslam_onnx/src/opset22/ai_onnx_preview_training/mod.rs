#![allow(unused)]

//! 详细说明[https://github.com/onnx/onnx/blob/main/docs/Operators.md]

// Compute one iteration of ADAGRAD, a stochastic gradient based optimization algorithm. This operator can conduct the optimization of multiple tensor variables.
// 1. 计算一次基于随机梯度的优化算法ADAGRAD的迭代。这个操作符可以对多个张量变量进行优化。
pub mod op_adagrad;

// Compute one iteration of Adam, a stochastic gradient based optimization algorithm. This operator can conduct the optimization of multiple tensor variables.
// 2. 计算一次基于随机梯度的优化算法Adam的迭代。这个操作符可以对多个张量变量进行优化。
pub mod op_adam;

// Gradient operator computes the partial derivatives of a specific tensor w.r.t. some other tensors.
// 3. 梯度算子计算一个特定张量相对于其他一些张量的偏导数。
pub mod op_gradient;

// Compute one iteration of stochastic gradient update with momentum. This operator can conduct the optimization of multiple tensor variables.
// 4. 计算一次带动量的随机梯度更新迭代。这个操作符可以对多个张量变量进行优化。
pub mod op_momentum;
