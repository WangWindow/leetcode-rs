/// 3100. Water Bottles II 换水问题 II
/// https://leetcode.com/problems/water-bottles-ii/
///
/// 给你两个整数 `numBottles` 和 `numExchange` 。
///
/// `numBottles` 代表你最初拥有的满水瓶数量。在一次操作中，你可以执行以下操作之一：
///
/// * 喝掉任意数量的满水瓶，使它们变成空水瓶。
/// * 用 `numExchange` 个空水瓶交换一个满水瓶。然后，将 `numExchange` 的值增加 1 。
///
/// 注意，你不能使用相同的 `numExchange` 值交换多批空水瓶。例如，如果 `numBottles == 3` 并且 `numExchange == 1` ，则不能用 `3` 个空水瓶交换成 `3` 个满水瓶。
///
/// 返回你 **最多** 可以喝到多少瓶水。
///
/// **示例 1：**
///
/// ![](https://assets.leetcode.com/uploads/2024/01/28/exampleone1.png)
///
/// ```
/// 输入：numBottles = 13, numExchange = 6
/// 输出：15
/// 解释：上表显示了满水瓶的数量、空水瓶的数量、numExchange 的值，以及累计喝掉的水瓶数量。
///
/// ```
///
/// **示例 2：**
///
/// ![](https://assets.leetcode.com/uploads/2024/01/28/example231.png)
///
/// ```
/// 输入：numBottles = 10, numExchange = 3
/// 输出：13
/// 解释：上表显示了满水瓶的数量、空水瓶的数量、numExchange 的值，以及累计喝掉的水瓶数量。
/// ```
///
/// **提示：**
///
/// * `1 <= numBottles <= 100 `
/// * `1 <= numExchange <= 100`

pub struct Solution;

impl Solution {
    /// 给你两个整数 `numBottles` 和 `numExchange` 。
    /// `numBottles` 代表你最初拥有的满水瓶数量。在一次操作中，你可以执行以下操作之一：
    ///
    /// - 喝掉任意数量的满水瓶，使它们变成空水瓶。
    /// - 用 `numExchange` 个空水瓶交换一个满水瓶。然后，将 `numExchange` 的值增加 1 。
    ///
    /// 注意，你不能使用相同的 `numExchange` 值交换多批空水瓶。
    /// 例如，如果 `numBottles` == 3 并且 `numExchange` == 1 ，则不能用 3 个空水瓶交换成 3 个满水瓶。
    ///
    /// 返回你 `最多` 可以喝到多少瓶水。
    ///
    /// 提示：
    /// `1 <= numBottles <= 100`;
    /// `1 <= numExchange <= 100`
    pub fn max_bottles_drunk(num_bottles: i32, num_exchange: i32) -> i32 {
        //* 就是说每次交换所需的空瓶数不一样（假设为递增，因为从少变多进行兑换消耗的空瓶数最少）
        //* 那么就喝完后再进行兑换
        //* 由于数据范围很小，所以使用模拟即可

        // 满瓶，空瓶，喝掉的瓶数，当前兑换所需空瓶数
        let mut full = num_bottles;
        let mut empty = 0;
        let mut drunk = 0;
        let mut exchange = num_exchange;

        while full > 0 {
            // 喝掉当前所有满瓶
            drunk += full;
            empty += full;
            full = 0;
            // 然后尽可能多的进行兑换
            while empty >= exchange {
                empty -= exchange;
                full += 1;
                exchange += 1;
            }
        }
        drunk
    }
}
