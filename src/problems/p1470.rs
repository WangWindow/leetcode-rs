/// 1470. Shuffle the Array 重新排列数组
/// https://leetcode.com/problems/shuffle-the-array/
///
/// 给你一个数组 `nums` ，数组中有 `2n` 个元素，按 `[x<sub>1</sub>,x<sub>2</sub>,...,x<sub>n</sub>,y<sub>1</sub>,y<sub>2</sub>,...,y<sub>n</sub>]` 的格式排列。
///
/// 请你将数组按 `[x<sub>1</sub>,y<sub>1</sub>,x<sub>2</sub>,y<sub>2</sub>,...,x<sub>n</sub>,y<sub>n</sub>]` 格式重新排列，返回重排后的数组。
///
/// **示例 1：**
///
/// ```
/// 输入：nums = [2,5,1,3,4,7], n = 3
/// 输出：[2,3,5,4,1,7]
/// 解释：由于 x1=2, x2=5, x3=1, y1=3, y2=4, y3=7 ，所以答案为 [2,3,5,4,1,7]
///
/// ```
///
/// **示例 2：**
///
/// ```
/// 输入：nums = [1,2,3,4,4,3,2,1], n = 4
/// 输出：[1,4,2,3,3,2,4,1]
///
/// ```
///
/// **示例 3：**
///
/// ```
/// 输入：nums = [1,1,2,2], n = 2
/// 输出：[1,2,1,2]
///
/// ```
///
/// **提示：**
///
/// * `1 <= n <= 500`
/// * `nums.length == 2n`
/// * `1 <= nums[i] <= 10^3`

pub struct Solution;

impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        // 创建一个新的数组来存储结果
        let mut ans: Vec<i32> = Vec::with_capacity(n as usize);

        // 遍历前 n 个元素和后 n 个元素，按照要求重新排列
        for i in 0..n as usize {
            ans.push(nums[i]);
            ans.push(nums[i + n as usize]);
        }

        ans
    }
}
