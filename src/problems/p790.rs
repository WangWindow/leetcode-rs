///! 790. Domino and Tromino Tiling
///! https://leetcode.com/problems/domino-and-tromino-tiling/
use crate::Problem;
pub struct Solution;

impl Problem for Solution {
    const TITLE: &'static str = "多米诺和托米诺平铺 (Domino and Tromino Tiling)";
    fn run() {
        let n = 30;
        let result = Solution::num_tilings(n);
        println!("790 => {}", result);
    }
}

impl Solution {
    /// 你正在做一个平铺游戏，游戏板的宽度为 2，长度为 n 。
    /// 你有三种不同形状的瓷砖：
    /// - 2 x 1 的多米诺瓷砖。
    /// - "L" 形的多米诺瓷砖。
    /// 计算有多少种方法可以平铺这个 2 x n 的游戏板。返回对 (10^9 + 7) 取模的结果。
    pub fn num_tilings(n: i32) -> i32 {
        //* 这个问题我大一数据结构课设做过类似的“铺地砖”问题，要使用动态规划
        //* 注：大一做的那题更难一点 😜
        // dp[i]表示铺满 2 x i 的方案数
        // 列出状态转移方程：
        // 如果最后一列是竖着放的多米诺瓷砖，则前面有 dp[i-1] 种铺法
        // 如果最后一列是横着放的多米诺瓷砖，则前面一列也必须是横着放的多米诺瓷砖，则前面有 dp[i-2] 种铺法
        // 如果最后一列是 "L" 形的多米诺瓷砖，则前面一列可以是竖着放的多米诺瓷砖，或者是横着放的多米诺瓷砖
        // 如果前面一列是竖着放的多米诺瓷砖，则前面有 dp[i-3] 种铺法
        // 如果前面一列是横着放的多米诺瓷砖，则前面两列也必须是横着放的多米诺瓷砖，则前面有 dp[i-4] 种铺法
        // 以此类推
        // dp[i] = dp[i-1] + dp[i-2] + 2*(dp[i-3] + ... + dp[1])
        // 则：
        // dp[i-1] = dp[i-2] + dp[i-3] + 2*(dp[i-4] + ... + dp[1])
        // 两式相减得：
        // dp[i] = 2*dp[i-1] + dp[i-3]
        // 这样就可以用 O(n) 的时间复杂度和 O(1) 的空间复杂度解决问题
        // dp[1] = 1, dp[2] = 2, dp[3] = 5
        const MOD: i64 = 1_000_000_007;
        match n {
            1 => 1,
            2 => 2,
            3 => 5,
            _ => {
                let mut dp: [i64; 3] = [1, 2, 5];
                for _ in 4..=n {
                    let next = (2 * dp[2] + dp[0]) % MOD;
                    dp[0] = dp[1];
                    dp[1] = dp[2];
                    dp[2] = next;
                }
                (dp[2] % MOD) as i32
            }
        }
    }
}
