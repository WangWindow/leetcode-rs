/// 1128. Number of Equivalent Domino Pairs 等价多米诺骨牌对的数量
/// https://leetcode.com/problems/number-of-equivalent-domino-pairs/
///
/// 给你一组多米诺骨牌 `dominoes` 。
///
/// 形式上，`dominoes[i] = [a, b]` 与 `dominoes[j] = [c, d]` **等价** 当且仅当 (`a == c` 且 `b == d`) 或者 (`a == d` 且 `b == c`) 。即一张骨牌可以通过旋转 `0` 度或 `180` 度得到另一张多米诺骨牌。
///
/// 在 `0 <= i < j < dominoes.length` 的前提下，找出满足 `dominoes[i]` 和 `dominoes[j]` 等价的骨牌对 `(i, j)` 的数量。
///
/// **示例 1：**
///
/// ```
/// 输入：dominoes = [[1,2],[2,1],[3,4],[5,6]]
/// 输出：1
///
/// ```
///
/// **示例 2：**
///
/// ```
/// 输入：dominoes = [[1,2],[1,2],[1,1],[1,2],[2,2]]
/// 输出：3
///
/// ```
///
/// **提示：**
///
/// * `1 <= dominoes.length <= 4 * 10<sup>4</sup>`
/// * `dominoes[i].length == 2`
/// * `1 <= dominoes[i][j] <= 9`

pub struct Solution;

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
