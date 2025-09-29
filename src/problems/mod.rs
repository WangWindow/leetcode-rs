//! 题目模块与注册表
// 此文件现在通过 build.rs 自动生成问题模块与注册表。
// 你只需要在 `src/problems/` 下新增形如 `p{数字}.rs` 的文件，
// 且在文件中提供 `pub fn run()`，即可被自动收集。

// 将生成的代码包含进来
include!(concat!(env!("OUT_DIR"), "/problems_registry.rs"));
