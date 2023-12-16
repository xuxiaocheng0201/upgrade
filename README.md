# Upgrade

[![Crate](https://img.shields.io/crates/v/upgrade.svg)](https://crates.io/crates/upgrade)
[![GitHub last commit](https://img.shields.io/github/last-commit/xuxiaocheng0201/upgrade)](https://github.com/xuxiaocheng0201/upgrade/commits/master)
[![GitHub issues](https://img.shields.io/github/issues-raw/xuxiaocheng0201/upgrade)](https://github.com/xuxiaocheng0201/upgrade/issues)
[![GitHub pull requests](https://img.shields.io/github/issues-pr/xuxiaocheng0201/upgrade)](https://github.com/xuxiaocheng0201/upgrade/pulls)
[![GitHub](https://img.shields.io/github/license/xuxiaocheng0201/upgrade)](https://github.com/xuxiaocheng0201/upgrade/blob/master/LICENSE)

**Read this in other languages: [English](README.md), [简体中文](README_zh.md).**

**Welcome to submit [PR](https://github.com/xuxiaocheng0201/variable-len-reader/pulls)!**

# Description

A Rust crate to upgrade your program easily.


# Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
upgrade = "*"
```


# Example

```rust
use upgrade::upgrade;

fn main() {
    upgrade("./upgrade.exe").unwrap();
}
```