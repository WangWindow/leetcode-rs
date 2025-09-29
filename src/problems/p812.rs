/// 812. Largest Triangle Area
/// https://leetcode.com/problems/largest-triangle-area/
use crate::{Problem, Solution};
pub struct Type;

impl Problem for Type {
    const ID: &'static str = "812";
    const TITLE: &'static str = "最大三角形面积 (Largest Triangle Area)";
    fn run() {
        let points = vec![vec![0, 0], vec![0, 1], vec![1, 0], vec![0, 2], vec![2, 0]];
        let area = Solution::largest_triangle_area(points);
        println!("812 => {:.5}", area);
    }
}

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
