//! 库入口：导出题目模块
pub mod problems;

/// 打印当前题库概览与推荐工作流
pub fn print_overview() {
    println!("推荐工作流：");
    println!("- 使用 ./leetcode-creator add <题号> 抓取并生成新题");
    println!("- 使用 ./leetcode-creator update <题号> 更新题目头部元数据");
    println!("- 使用 cargo test 编写和运行单元测试");
    println!();
    println!("当前已收录 {} 道题。", problems::IDS.len());
}
