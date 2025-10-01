///! 1518. Water Bottles
///! https://leetcode.com/problems/water-bottles/
use crate::Problem;
pub struct Solution;

impl Problem for Solution {
    const TITLE: &'static str = "换水问题 (Water Bottles)";
    fn run() {
        let num_bottles = 9;
        let num_exchange = 3;
        let result = Solution::num_water_bottles(num_bottles, num_exchange);
        println!("1518 => {}", result);
    }
}

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
