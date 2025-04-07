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

//! 八叉树占据地图 

// 1. PointCloud模块
pub mod PointCloud;

// 2. ScanGraph模块
pub mod ScanGraph;

// 3. OcTree模块

// pub mod OcTree;
// 这里使用现成的八叉树数据结构, 仅对地图作适配
// Features:
// Unsigned arithmetics, bitwise operations.
// Tree structure is represented by flat, reusable pools. Removed data is marked only.
// Few memory allocations. Smallvec and Heapless structures are used.
// No smart pointers (Rc, RefCell e.t.c)
// Able to operate with Position or Volume data.
// Could be used with the Bevy game engine or as a standalone tree.
pub mod OkTree;

// 4. Types模块(使用nalgebra库)
pub mod Types;

// 5. Utils模块
pub mod Utils;

// 6. Map模块
pub mod Map;