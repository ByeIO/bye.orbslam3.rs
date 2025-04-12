# `deno_whoami`

[![](https://img.shields.io/crates/v/deno_whoami.svg)](https://crates.io/crates/deno_whoami)

This crate implements the username() API for the Node compat layer in Deno. It uses `getpwuid_r` on Unix and fallbacks for `whoami` crate on Windows.