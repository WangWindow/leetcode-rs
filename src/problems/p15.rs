/// 15. 3Sum 三数之和
/// https://leetcode.com/problems/3sum/
///
/// 给你一个整数数组 `nums` ，判断是否存在三元组 `[nums[i], nums[j], nums[k]]` 满足 `i != j`、`i != k` 且 `j != k` ，同时还满足 `nums[i] + nums[j] + nums[k] == 0` 。请你返回所有和为 `0` 且不重复的三元组。
///
/// **注意：**答案中不可以包含重复的三元组。
///
/// **示例 1：**
///
/// ```
/// 输入：nums = [-1,0,1,2,-1,-4]
/// 输出：[[-1,-1,2],[-1,0,1]]
/// 解释：
/// nums[0] + nums[1] + nums[2] = (-1) + 0 + 1 = 0 。
/// nums[1] + nums[2] + nums[4] = 0 + 1 + (-1) = 0 。
/// nums[0] + nums[3] + nums[4] = (-1) + 2 + (-1) = 0 。
/// 不同的三元组是 [-1,0,1] 和 [-1,-1,2] 。
/// 注意，输出的顺序和三元组的顺序并不重要。
///
/// ```
///
/// **示例 2：**
///
/// ```
/// 输入：nums = [0,1,1]
/// 输出：[]
/// 解释：唯一可能的三元组和不为 0 。
///
/// ```
///
/// **示例 3：**
///
/// ```
/// 输入：nums = [0,0,0]
/// 输出：[[0,0,0]]
/// 解释：唯一可能的三元组和为 0 。
///
/// ```
///
/// **提示：**
///
/// * `3 <= nums.length <= 3000`
/// * `-10<sup>5</sup> <= nums[i] <= 10<sup>5</sup>`

pub struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        // 看着应该是 “两数之和” 这个问题的升级版
        // 那么我们同样使用 “双指针” 思路
        // 不过返回的不是索引了，所以不用构建索引数组

        // 对数组进行排序（注意不要去重，因为存在【-1, -1, 2】这样的数据）
        let mut nums = nums.clone();
        nums.sort_unstable();

        // 初始化结果数组
        let mut result = Vec::new();

        // 那么让我们开始吧！
        for i in 0..nums.len() {
            // 如果最左边数字已经大于 0 了，那么后续的肯定也大于 0，那么直接结束
            if nums[i] > 0 {
                break;
            }

            // 去重：如果当前数字和上一个数字相同，那么就遍历下一个 i
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            // 初始化左右指针
            let mut p_l = i + 1;
            let mut p_r = nums.len() - 1;

            while p_l < p_r {
                let sum = nums[i] + nums[p_l] + nums[p_r];

                if sum == 0 {
                    result.push(vec![nums[i], nums[p_l], nums[p_r]]);

                    p_l += 1;
                    p_r -= 1;

                    // 去重
                    // 如果中间的数字和下一个数字相同，那么就跳过
                    while p_l < p_r && nums[p_l] == nums[p_l - 1] {
                        p_l += 1;
                    }
                    // 如果右边的数字和上一个数字相同，那么就跳过
                    while p_l < p_r && nums[p_r] == nums[p_r + 1] {
                        p_r -= 1;
                    }
                } else if sum < 0 {
                    p_l += 1;
                } else {
                    p_r -= 1;
                }
            }
        }
        result
    }
}

#[cfg(test)]
fn test_three_sum() {
    let nums = vec![-1, 0, 1, 2, -1, -4];
    let result = Solution::three_sum(nums);
    assert_eq!(result, vec![vec![-1, -1, 2], vec![-1, 0, 1]]);

    let nums = vec![0, 1, 1];
    let result = Solution::three_sum(nums);
    assert_eq!(result, vec![vec![]]);

    let nums = vec![0, 0, 0];
    let result = Solution::three_sum(nums);
    assert_eq!(result, vec![vec![0, 0, 0]]);
}
