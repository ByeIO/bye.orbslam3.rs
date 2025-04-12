# Error Reporter

[![crates.io](https://img.shields.io/crates/v/error_reporter.svg)](http://crates.io/crates/error_reporter)
[![docs.rs](https://docs.rs/error_reporter/badge.svg)](http://docs.rs/error_reporter)

This crate contains a copy of the `std::error::Report` type which is currently unstable.
The copy is identical except that backtraces are not supported since they rely on other unstable features.
