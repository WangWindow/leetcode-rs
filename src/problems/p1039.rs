/// 1039. Minimum Score Triangulation of Polygon 多边形三角剖分的最低得分
/// https://leetcode.com/problems/minimum-score-triangulation-of-polygon/
///
/// 你有一个凸的 `n` 边形，其每个顶点都有一个整数值。给定一个整数数组 `values` ，其中 `values[i]` 是按 **顺时针顺序** 第 `i` 个顶点的值。
///
/// 假设将多边形 **剖分** 为 `n - 2` 个三角形。对于每个三角形，该三角形的值是顶点标记的**乘积**，三角剖分的分数是进行三角剖分后所有 `n - 2` 个三角形的值之和。
///
/// 返回 *多边形进行三角剖分后可以得到的最低分* 。
///
/// **示例 1：**
///
/// **![](https://assets.leetcode.com/uploads/2025/10/23/ex0-2.png)**
///
/// ```
/// 输入：values = [1,2,3]
/// 输出：6
/// 解释：多边形已经三角化，唯一三角形的分数为 6。
///
/// ```
///
/// **示例 2：**
///
/// ![](https://assets.leetcode.com/uploads/2025/10/23/ex1-2.png)
///
/// ```
/// 输入：values = [3,7,4,5]
/// 输出：144
/// 解释：有两种三角剖分，可能得分分别为：3*7*5 + 4*5*7 = 245，或 3*4*5 + 3*4*7 = 144。最低分数为 144。
///
/// ```
///
/// **示例 3：**
///
/// **![](https://assets.leetcode.com/uploads/2025/10/23/ex2.png)**
///
/// ```
/// 输入：values = [1,3,1,4,1,5]
/// 输出：13
/// 解释：最低分数三角剖分的得分情况为 1*1*3 + 1*1*4 + 1*1*5 + 1*1*1 = 13。
///
/// ```
///
/// **提示：**
///
/// * `n == values.length`
/// * `3 <= n <= 50`
/// * `1 <= values[i] <= 100`

pub struct Solution;

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
