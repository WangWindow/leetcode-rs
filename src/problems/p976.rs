//! 976. Largest Perimeter Triangle
//! https://leetcode.com/problems/largest-perimeter-triangle/

use crate::Solution;

pub fn run() {
    let nums = vec![2, 1, 2];
    let perimeter = Solution::largest_perimeter(nums);
    println!("976 => {}", perimeter);
}

impl Solution {
    // 给定边长数组，求能组成的最大三角形周长
    pub fn largest_perimeter(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable_by(|a, b| b.cmp(a)); // 降序排序
        let n = nums.len();
        for i in 0..n - 2 {
            // 从大到小枚举三条边
            if nums[i] < nums[i + 1] + nums[i + 2] {
                // 满足三角形两边之和大于第三边
                return nums[i] + nums[i + 1] + nums[i + 2];
            }
        }
        0 // 无法组成三角形
    }
}
