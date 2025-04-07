#![allow(unused)]

//! The `wasmtime` command line tool.
//!
//! Primarily used to run WebAssembly modules.
//! See `wasmtime --help` for usage.

use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    return wasmtime_cli::Wasmtime::parse().execute();
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    wasmtime_cli::Wasmtime::command().debug_assert()
}
