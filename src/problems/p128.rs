/// 128. Longest Consecutive Sequence 最长连续序列
/// https://leetcode.com/problems/longest-consecutive-sequence/
///
/// 给定一个未排序的整数数组 `nums` ，找出数字连续的最长序列（不要求序列元素在原数组中连续）的长度。
///
/// 请你设计并实现时间复杂度为 `O(n)` 的算法解决此问题。
///
/// **示例 1：**
///
/// ```
/// 输入：nums = [100,4,200,1,3,2]
/// 输出：4
/// 解释：最长数字连续序列是 [1, 2, 3, 4]。它的长度为 4。
/// ```
///
/// **示例 2：**
///
/// ```
/// 输入：nums = [0,3,7,2,5,8,4,6,0,1]
/// 输出：9
///
/// ```
///
/// **示例 3：**
///
/// ```
/// 输入：nums = [1,0,1,2]
/// 输出：3
///
/// ```
///
/// **提示：**
///
/// * `0 <= nums.length <= 10<sup>5</sup>`
/// * `-10<sup>9</sup> <= nums[i] <= 10<sup>9</sup>`

pub struct Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        // 要求时间复杂度为 O(n), 所以采用空间换时间的方式
        // 使用 HashSet 来存储所有数字，以便快速查找
        // 并且 HashSet 可以去重

        let num_set: std::collections::HashSet<i32> = nums.into_iter().collect();
        let mut longest = 0;

        for &num in &num_set {
            // 匹配到一个序列的起点（即 num - 1 不在集合中），然后向上查找连续的数字
            // 哈希集合的查找是 O(1) 的，所以整体时间复杂度为 O(n)
            if !num_set.contains(&(num - 1)) {
                let mut current_num = num;
                let mut current_len = 1;

                // 一直查找当前数字的下一个连续数字，直到找不到为止
                while num_set.contains(&(current_num + 1)) {
                    current_num += 1;
                    current_len += 1;
                }

                // 记录最长的长度
                longest = longest.max(current_len);
            }
        }

        longest
    }
}

#[test]
fn test_longest_consecutive() {
    let nums = vec![100, 4, 200, 1, 3, 2];
    assert_eq!(Solution::longest_consecutive(nums), 4);

    let nums = vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1];
    assert_eq!(Solution::longest_consecutive(nums), 9);

    let nums = vec![1, 0, 1, 2];
    assert_eq!(Solution::longest_consecutive(nums), 3);
}
