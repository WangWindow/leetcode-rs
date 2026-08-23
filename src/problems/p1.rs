/// 1. Two Sum 两数之和
/// https://leetcode.com/problems/two-sum/
///
/// 给定一个整数数组 `nums` 和一个整数目标值 `target`，请你在该数组中找出 **和为目标值** *`target`* 的那 **两个** 整数，并返回它们的数组下标。
///
/// 你可以假设每种输入只会对应一个答案，并且你不能使用两次相同的元素。
///
/// 你可以按任意顺序返回答案。
///
/// **示例 1：**
///
/// ```
/// 输入：nums = [2,7,11,15], target = 9
/// 输出：[0,1]
/// 解释：因为 nums[0] + nums[1] == 9 ，返回 [0, 1] 。
///
/// ```
///
/// **示例 2：**
///
/// ```
/// 输入：nums = [3,2,4], target = 6
/// 输出：[1,2]
///
/// ```
///
/// **示例 3：**
///
/// ```
/// 输入：nums = [3,3], target = 6
/// 输出：[0,1]
///
/// ```
///
/// **提示：**
///
/// * `2 <= nums.length <= 10<sup>4</sup>`
/// * `-10<sup>9</sup> <= nums[i] <= 10<sup>9</sup>`
/// * `-10<sup>9</sup> <= target <= 10<sup>9</sup>`
/// * **只会存在一个有效答案**
///
/// **进阶：** 你可以想出一个时间复杂度小于 `O(n<sup>2</sup>)` 的算法吗？

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // 采用双指针
        // 先对数组进行排序，然后使用两个指针分别指向数组的开头和结尾
        //
        // 计算它们的和：
        // 如果和小于目标值，则左指针右移；
        // 如果和大于目标值，则右指针左移。
        // 直到找到目标值为止。

        // 构造一个索引数组，保存原始数组的索引和值的对应关系 【(下标，值)...】
        let mut indexed_nums = nums.iter().enumerate().collect::<Vec<(usize, &i32)>>();
        // 按照值进行排序
        indexed_nums.sort_unstable_by(|a, b| a.1.cmp(b.1));

        // 左右指针，用于遍历索引数组
        let mut p_l = 0;
        let mut p_r = nums.len() - 1;

        while p_l < p_r {
            let sum = indexed_nums[p_l].1 + indexed_nums[p_r].1;

            match sum.cmp(&target) {
                std::cmp::Ordering::Less => p_l += 1,
                std::cmp::Ordering::Greater => p_r -= 1,
                std::cmp::Ordering::Equal => {
                    return vec![indexed_nums[p_l].0 as i32, indexed_nums[p_r].0 as i32];
                }
            }
        }

        vec![]
    }
}

#[test]
fn test_two_sum() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let result = Solution::two_sum(nums, target);
    assert_eq!(result, vec![0, 1]);
}
