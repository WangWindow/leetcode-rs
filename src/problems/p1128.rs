///! 1128. Number of Equivalent Domino Pairs
///! https://leetcode.com/problems/number-of-equivalent-domino-pairs/
use crate::Problem;
pub struct Solution;

impl Problem for Solution {
    const TITLE: &'static str = "等价多米诺骨牌对的数量 (Number of Equivalent Domino Pairs)";
    fn run() {
        let dominoes = vec![vec![1, 2], vec![2, 1], vec![3, 4], vec![5, 6]];
        let result = Solution::num_equiv_domino_pairs(dominoes);
        println!("1128 => {}", result);
    }
}

impl Solution {
    /// 给定一个多米诺骨牌数组 dominoes，返回其中等价骨牌对的数量。
    /// 如果一张骨牌 [a, b] 等价于另一张骨牌 [c, d]，那么 a == c 且 b == d，或者 a == d 且 b == c。
    /// 例如，[1, 2] 和 [2, 1] 是等价的。
    pub fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
        // 使用一个长度为 100 的数组来记录每种骨牌的出现次数
        // 即使用哈希表进行处理，哈希函数为：min(x,y)*10+max(x,y)
        let mut count = vec![0; 100];
        let mut res = 0;
        for d in dominoes {
            let key = d[0].min(d[1]) * 10 + d[0].max(d[1]);
            res += count[key as usize];
            count[key as usize] += 1;
        }
        res
    }
}
