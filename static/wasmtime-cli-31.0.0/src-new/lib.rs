//! The Wasmtime command line interface (CLI) crate.
//!
//! This crate implements the Wasmtime command line tools.

#![allow(missing_docs)]

// 命令解析
pub mod commands;

// 通用
#[cfg(feature = "run")]
pub(crate) mod common;

// cli顶层
pub mod cli;
