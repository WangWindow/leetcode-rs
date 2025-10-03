///! 407. Trapping Rain Water II
///! https://leetcode.com/problems/trapping-rain-water-ii/
use crate::Problem;
pub struct Solution;

impl Problem for Solution {
    const TITLE: &'static str = "接雨水 II (Trapping Rain Water II)";
    fn run() {
        let height_map = vec![
            vec![1, 4, 3, 1, 3, 2],
            vec![3, 2, 1, 3, 2, 4],
            vec![2, 3, 3, 2, 3, 1],
        ];
        let result = Self::trap_rain_water(height_map);
        println!("res: {}", result);
    }
}

impl Solution {
    /// 给你一个 `m x n` 的矩阵，其中的值均为非负整数，代表二维高度图每个单元的高度。
    /// 请计算图中形状最多能接多少体积的雨水。
    ///
    /// 提示:
    /// - m == heightMap.length
    /// - n == heightMap[i].length
    /// - 1 <= m, n <= 200
    /// - 0 <= heightMap[i][j] <= 2 * 10^4
    pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
        //* 注意到如果采用逐层遍历的方式，由于每一层的遍历都需要 O(mn) 的时间复杂度
        //* 因此整体的时间复杂度会达到 O(mnh)，其中 h 是高度图中的最大高度，这样的复杂度是无法接受的。
        //* 因此我们需要一种更高效的方式来计算每一层的积水量。
        //* 这里参考官方题解中的思路，使用优先队列（最小堆）来实现。
        use std::cmp::Reverse;
        let m = height_map.len();
        let n = height_map[0].len();
        let mut visited = vec![vec![false; n]; m];
        let mut heap = std::collections::BinaryHeap::new();
        let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut total = 0;
        for i in 0..m {
            visited[i][0] = true;
            visited[i][n - 1] = true;
            heap.push(Reverse((height_map[i][0], i, 0)));
            heap.push(Reverse((height_map[i][n - 1], i, n - 1)));
        }
        for j in 0..n {
            visited[0][j] = true;
            visited[m - 1][j] = true;
            heap.push(Reverse((height_map[0][j], 0, j)));
            heap.push(Reverse((height_map[m - 1][j], m - 1, j)));
        }
        while let Some(Reverse((height, x, y))) = heap.pop() {
            for &(dx, dy) in &directions {
                let nx = x.wrapping_add(dx as usize);
                let ny = y.wrapping_add(dy as usize);
                if nx < m && ny < n && !visited[nx][ny] {
                    visited[nx][ny] = true;
                    total += (height - height_map[nx][ny]).max(0);
                    heap.push(Reverse((height.max(height_map[nx][ny]), nx, ny)));
                }
            }
        }
        total
    }
}
