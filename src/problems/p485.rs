/// 485. Max Consecutive Ones 最大连续 1 的个数
/// https://leetcode.com/problems/max-consecutive-ones/
///
/// 给定一个二进制数组 `nums` ， 计算其中最大连续 `1` 的个数。
///
/// **示例 1：**
///
/// ```
/// 输入：nums = [1,1,0,1,1,1]
/// 输出：3
/// 解释：开头的两位和最后的三位都是连续 1 ，所以最大连续 1 的个数是 3.
///
/// ```
///
/// **示例 2:**
///
/// ```
/// 输入：nums = [1,0,1,1,0,1]
/// 输出：2
///
/// ```
///
/// **提示：**
///
/// * `1 <= nums.length <= 10<sup>5</sup>`
/// * `nums[i]` 不是 `0` 就是 `1`.

pub struct Solution;

impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut max_count = 0; // 记录最大连续 1 的个数
        let mut current_count = 0; // 记录当前连续 1 的个数
        for num in nums {
            match num {
                1 => current_count += 1, // 遇到 1，当前连续 1 的个数加 1
                0 => current_count = 0,  // 遇到 0，当前连续 1 的个数重置为 0
                _ => unreachable!(),
            }
            // 更新最大连续 1 的个数
            max_count = max_count.max(current_count);
        }

        max_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_max_consecutive_ones() {
        assert_eq!(
            Solution::find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 1]),
            3
        );
        assert_eq!(
            Solution::find_max_consecutive_ones(vec![1, 0, 1, 1, 0, 1]),
            2
        );
    }
}
