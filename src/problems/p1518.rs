/// 1518. Water Bottles 换水问题
/// https://leetcode.com/problems/water-bottles/
///
/// 超市正在促销，你可以用 `numExchange` 个空水瓶从超市兑换一瓶水。最开始，你一共购入了 `numBottles` 瓶水。
///
/// 如果喝掉了水瓶中的水，那么水瓶就会变成空的。
///
/// 给你两个整数 `numBottles` 和 `numExchange` ，返回你 **最多** 可以喝到多少瓶水。
///
/// **示例 1：**
///
/// **![](https://assets.leetcode.cn/aliyun-lc-upload/uploads/2020/07/19/sample_1_1875.png)**
///
/// ```
/// 输入：numBottles = 9, numExchange = 3
/// 输出：13
/// 解释：你可以用 3 个空瓶兑换 1 瓶水。
/// 所以最多能喝到 9 + 3 + 1 = 13 瓶水。
///
/// ```
///
/// **示例 2：**
///
/// ![](https://assets.leetcode.cn/aliyun-lc-upload/uploads/2020/07/19/sample_2_1875.png)
///
/// ```
/// 输入：numBottles = 15, numExchange = 4
/// 输出：19
/// 解释：你可以用 4 个空瓶兑换 1 瓶水。
/// 所以最多能喝到 15 + 3 + 1 = 19 瓶水。
///
/// ```
///
/// **提示：**
///
/// * `1 <= numBottles <= 100`
/// * `2 <= numExchange <= 100`

pub struct Solution;

impl Solution {
    /// 给你 numBottles 瓶满的水瓶和一个整数 numExchange 。
    /// numExchange 表示你可以用 numExchange 个空瓶换一瓶满的水。
    pub fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
        // 每瓶的水的价格相当于是 (numExchange - 1) 个空瓶
        // 那么初始时所拥有的价值为 (numBottles * numExchange) 个空瓶
        // 最终剩余的空瓶无法再兑换，剩余的空瓶数量为 (1..numExchange - 1) 个空瓶
        // 故能喝到的水的区间为：
        // ((numBottles * numExchange - (numExchange - 1) ) / (numExchange - 1), (numBottles * numExchange - 1) / (numExchange - 1)]
        // 即：[(num_bottles * num_exchange - 1) / (num_exchange - 1)]
        (num_bottles * num_exchange - 1) / (num_exchange - 1)
    }
}
