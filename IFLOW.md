# IFLOW.md - ecgen-rs 项目指南

## 项目概述

`ecgen-rs` 是一个用 Rust 编写的枚举组合生成库（Enumerative Combinatoric Generation）。该项目提供了一系列高效的算法，用于生成各种组合对象，包括：

- 组合（Combinations）- 使用同质旋转门算法（homogeneous revolving-door algorithm）
- 置换（Permutations）- 包括邻位移换（Steinhaus-Johnson-Trotter 算法）和星置换（star transposition algorithm）
- 格雷码（Gray Codes）- 二进制反射格雷码（Binary Reflected Gray Code）
- 集合划分（Set Partitions）- 使用受限增长字符串（Restricted Growth String）表示
- 集合二分（Set Bipartitions）- 将集合划分为两个子集

该项目通过生成器（Generators）实现，使用 `genawaiter` 库，能够高效地生成这些组合结构，而无需一次性将所有结果存储在内存中。

## 构建和运行

### 安装
- 确保已安装 Rust 工具链（包括 Cargo）：[Rust 安装指南](https://www.rust-lang.org/tools/install)
- 安装此库：`cargo install ecgen-rs`

### 项目结构
- `src/lib.rs` - 库的主入口点，导出所有模块
- `src/combin.rs` - 组合和组合生成器
- `src/gray_code.rs` - 格雷码生成器
- `src/perm.rs` - 置换和置换生成器
- `src/set_bipart.rs` - 集合二分生成器
- `src/set_partition.rs` - 集合划分生成器
- `Cargo.toml` - 项目依赖和元数据

### 依赖
- `genawaiter` - 用于实现异步生成器
- `criterion` - 用于基准测试（开发依赖）

### 测试
- 运行所有测试：`cargo test`
- 项目包含多个单元测试，验证各种组合生成器的正确性

## 开发约定

### 编码风格
- 遵循 Rust 的标准编码规范
- 使用 `const fn` 来实现纯函数，如 `factorial`、`comb` 和 `stirling2nd`
- 代码中包含丰富的文档注释和示例

### 模块化
- 代码按功能分成多个模块：`combin`、`gray_code`、`perm`、`set_bipart`、`set_partition`
- 每个模块负责其特定的组合生成算法

### 生成器模式
- 使用 `genawaiter` 库实现惰性生成器，用于高效生成大量组合对象
- 生成器返回 `GenBoxed<T>` 类型，支持 `for` 循环迭代

### 算法实现
- 实现了经典的组合生成算法，如 Steinhaus-Johnson-Trotter（SJT）、Ehrlich（星置换）、格雷码、以及集合划分的递归算法
- 代码详细注释了算法的逻辑和参考文献

## 许可证

该项目采用 MIT 许可证或 Apache 2.0 许可证双许可模式（dual licensed）。

## 贡献

遵循标准的 Rust 贡献流程，详见 `CONTRIBUTING.md`。