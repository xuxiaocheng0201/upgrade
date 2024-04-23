# 便捷更新

[![Crate](https://img.shields.io/crates/v/upgrade.svg)](https://crates.io/crates/upgrade)
[![GitHub last commit](https://img.shields.io/github/last-commit/xuxiaocheng0201/upgrade)](https://github.com/xuxiaocheng0201/upgrade/commits/master)
[![GitHub issues](https://img.shields.io/github/issues-raw/xuxiaocheng0201/upgrade)](https://github.com/xuxiaocheng0201/upgrade/issues)
[![GitHub pull requests](https://img.shields.io/github/issues-pr/xuxiaocheng0201/upgrade)](https://github.com/xuxiaocheng0201/upgrade/pulls)
[![GitHub](https://img.shields.io/github/license/xuxiaocheng0201/upgrade)](https://github.com/xuxiaocheng0201/upgrade/blob/master/LICENSE)

**其他语言版本：[English](README.md)，[简体中文](README_zh.md)。**

# 描述

更方便地自动更新程序本身
（基于 [self-replace](https://crates.io/crates/self-replace) ）


# 用法

将以下内容添加到你的`Cargo.toml`：

```toml
[dependencies]
upgrade = "^2"
```


# 示例

```rust
use upgrade::upgrade;

fn main() {
    upgrade("./upgrade.exe").unwrap();
}
```

或者在启动时添加参数：

```rust,no_run
use upgrade::run_upgrade;

fn main() {
    run_upgrade("./upgrade.exe", true, ["--upgraded"]).unwrap();
}
```
