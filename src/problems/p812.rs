/// 812. Largest Triangle Area 最大三角形面积
/// https://leetcode.com/problems/largest-triangle-area/
///
/// 给你一个由 **X-Y** 平面上的点组成的数组 `points` ，其中 `points[i] = [x<sub>i</sub>, y<sub>i</sub>]` 。从其中取任意三个不同的点组成三角形，返回能组成的最大三角形的面积。与真实值误差在 `10<sup>-5</sup>` 内的答案将会视为正确答案**。**
///
/// **示例 1：**
///
/// ![](https://s3-lc-upload.s3.amazonaws.com/uploads/2018/04/04/1027.png)
///
/// ```
/// 输入：points = [[0,0],[0,1],[1,0],[0,2],[2,0]]
/// 输出：2.00000
/// 解释：输入中的 5 个点如上图所示，红色的三角形面积最大。
///
/// ```
///
/// **示例 2：**
///
/// ```
/// 输入：points = [[1,0],[0,0],[0,1]]
/// 输出：0.50000
///
/// ```
///
/// **提示：**
///
/// * `3 <= points.length <= 50`
/// * `-50 <= x<sub>i</sub>, y<sub>i</sub> <= 50`
/// * 给出的所有点 **互不相同**

pub struct Solution;

impl Solution {
    /// 给定点集，求由其中任意三个点组成的三角形的最大面积
    pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
        let n = points.len();
        let mut best = 0f64;
        for i in 0..n {
            for j in i + 1..n {
                for k in j + 1..n {
                    let a = &points[i];
                    let b = &points[j];
                    let c = &points[k];
                    // 利用向量叉积公式计算三角形面积：| (b-a) x (c-a) | / 2
                    let area = (((b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0]))
                        .abs() as f64)
                        / 2.0;
                    if area > best {
                        best = area;
                    }
                }
            }
        }
        best
    }
}
