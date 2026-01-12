///! {题号}. {题目英文名}
///! {题目链接}
use crate::Problem;
pub struct Solution;

impl Problem for Solution {
    const TITLE: &'static str = "{题目中文名} ({题目英文名})";
    fn run() {
        // 示例输入
        let n = 0;
        let result = Solution::your_method(n);
        println!("{题号} => {}", result);
    }
}

impl Solution {
    pub fn your_method(n: i32) -> i32 {
        // TODO: 实现算法
        0
    }
}
