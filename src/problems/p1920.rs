///! 1920. Build Array from Permutation
///! https://leetcode.com/problems/build-array-from-permutation/
use crate::Problem;
pub struct Solution;

impl Problem for Solution {
    const TITLE: &'static str = "基于排列构建数组 (Build Array from Permutation)";
    fn run() {
        let nums = vec![0, 2, 1, 5, 3, 4];
        let result = Solution::build_array(nums);
        println!("1920 => {:?}", result);
    }
}

impl Solution {
    /// 给你一个 从 0 开始的排列 nums（下标也从 0 开始）。
    /// 请你构建一个 同样长度 的数组 ans ，对于每个 i（0 <= i < nums.length），都满足 ans[i] = nums[nums[i]] 。
    /// 返回构建好的数组 ans 。
    pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
        //? 送分题 ?
        nums.iter().map(|&i| nums[i as usize]).collect()
    }
}
