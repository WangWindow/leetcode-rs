/// 976. Largest Perimeter Triangle 三角形的最大周长
/// https://leetcode.com/problems/largest-perimeter-triangle/
///
/// 给定由一些正数（代表长度）组成的数组 `nums` ，返回 *由其中三个长度组成的、**面积不为零**的三角形的最大周长* 。如果不能形成任何面积不为零的三角形，返回 `0`。
///
/// **示例 1：**
///
/// ```
/// 输入：nums = [2,1,2]
/// 输出：5
/// 解释：你可以用三个边长组成一个三角形:1 2 2。
///
/// ```
///
/// **示例 2：**
///
/// ```
/// 输入：nums = [1,2,1,10]
/// 输出：0
/// 解释：
/// 你不能用边长 1,1,2 来组成三角形。
/// 不能用边长 1,1,10 来构成三角形。
/// 不能用边长 1、2 和 10 来构成三角形。
/// 因为我们不能用任何三条边长来构成一个非零面积的三角形，所以我们返回 0。
/// ```
///
/// **提示：**
///
/// * `3 <= nums.length <= 10<sup>4</sup>`
/// * `1 <= nums[i] <= 10<sup>6</sup>`

pub struct Solution;

impl Solution {
    /// 给定边长数组，求能组成的最大三角形周长
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
