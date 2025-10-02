///! 3100. Water Bottles II
///! https://leetcode.com/problems/water-bottles-ii/
use crate::Problem;
pub struct Solution;

impl Problem for Solution {
    const TITLE: &'static str = "换水问题II (Water Bottles II)";
    fn run() {
        let num_bottles = 13;
        let num_exchange = 6;
        let result = Solution::max_bottles_drunk(num_bottles, num_exchange);
        println!("3100 => {result}");
    }
}

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
