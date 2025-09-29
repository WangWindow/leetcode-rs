//! 库入口：导出题目模块与通用 `Solution` 结构体
pub mod problems;

use std::env;

// LeetCode 题解常用占位结构体，便于使用 `impl Solution` 实现静态方法
pub struct Solution;

/// 每道题需要实现的统一接口
pub trait Problem {
    const ID: &'static str;
    const TITLE: &'static str;
    fn run();
}

/// 按题号运行对应题目的入口：
/// 用法示例：
/// ```sh
///   cargo run -- 812
/// ```
/// 若不传参数，会列出可用题号。
pub fn run() {
    let mut args = env::args().skip(1);
    if let Some(id) = args.next() {
        match problems::dispatch(&id) {
            Some(run) => run(),
            None => {
                eprintln!("未找到题号: {id}");
                print_available();
            }
        }
    } else {
        print_available();
    }
}

/// 打印可用题号和用法说明
fn print_available() {
    println!("用法: cargo run -- <题号>");
    println!("可用题目:");
    for entry in problems::REGISTRY {
        println!("- {}: {}", entry.id, entry.title);
    }
}
