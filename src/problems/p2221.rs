///! 2221. Find Triangular Sum of an Array
///! https://leetcode.com/problems/find-triangular-sum-of-an-array/
use crate::Problem;
pub struct Solution;

impl Problem for Solution {
    const TITLE: &'static str = "数组的三角和 (Find Triangular Sum of an Array)";
    fn run() {
        let nums = vec![1, 2, 3, 4, 5];
        let result = Solution::triangular_sum(nums);
        println!("2221 => {}", result);
    }
}

impl Solution {
    /// 给定一个整数数组 nums，返回其三角和。
    /// 三角和的计算方法是：每次将数组中相邻的两个元素相加，形成一个新的数组，直到数组长度为 1。
    pub fn triangular_sum(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        for i in 0..n - 1 {
            for j in 0..n - 1 - i {
                nums[j] = (nums[j] + nums[j + 1]) % 10; // 只保留个位数
            }
        }
        nums[0]
    }
}
