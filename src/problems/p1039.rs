/// 1039. Minimum Score Triangulation of Polygon
/// https://leetcode.com/problems/minimum-score-triangulation-of-polygon/
use crate::Problem;
pub struct Solution;

impl Problem for Solution {
    const TITLE: &'static str = "多边形三角剖分的最低得分 (Minimum Score Triangulation of Polygon)";
    fn run() {
        let values: Vec<i32> = vec![1, 2, 3];
        let score = Solution::min_score_triangulation(values);
        println!("1039 => {}", score);
    }
}

impl Solution {
    /// 给定一个多边形的顶点值数组 values，返回将该多边形三角剖分后所有三角形的分数之和的最小值。
    pub fn min_score_triangulation(values: Vec<i32>) -> i32 {
        // 注意，此题不能使用排序，因为顶点顺序不能变（否则就不是同一个多边形了）

        // 考虑使用动态规划
        // dp[i][j] 表示从第 i 个顶点到第 j 个顶点组成的多边形的最小分数和
        // 则 dp[i][j] = min(dp[i][k] + dp[k][j] + values[i]*values[j]*values[k])，其中 i < k < j
        // 初始条件：dp[i][i+1] = 0，表示两个顶点无法组成三角形，分数为 0
        // 最终结果为 dp[0][n-1]
        let n = values.len();
        let mut dp = vec![vec![0; n]; n];
        for length in 2..n {
            for i in 0..n - length {
                let j = i + length;
                dp[i][j] = i32::MAX;
                for k in i + 1..j {
                    dp[i][j] =
                        dp[i][j].min(dp[i][k] + dp[k][j] + values[i] * values[j] * values[k]);
                }
            }
        }
        dp[0][n - 1]
    }
}
