/// 11. Container With Most Water 盛最多水的容器
/// https://leetcode.com/problems/container-with-most-water/
///
/// 给定一个长度为 `n` 的整数数组 `height` 。有 `n` 条垂线，第 `i` 条线的两个端点是 `(i, 0)` 和 `(i, height[i])` 。
///
/// 找出其中的两条线，使得它们与 `x` 轴共同构成的容器可以容纳最多的水。
///
/// 返回容器可以储存的最大水量。
///
/// **说明：**你不能倾斜容器。
///
/// **示例 1：**
///
/// ![](https://aliyun-lc-upload.oss-cn-hangzhou.aliyuncs.com/aliyun-lc-upload/uploads/2018/07/25/question_11.jpg)
///
/// ```
/// 输入：[1,8,6,2,5,4,8,3,7]
/// 输出：49
/// 解释：图中垂直线代表输入数组 [1,8,6,2,5,4,8,3,7]。在此情况下，容器能够容纳水（表示为蓝色部分）的最大值为 49。
/// ```
///
/// **示例 2：**
///
/// ```
/// 输入：height = [1,1]
/// 输出：1
///
/// ```
///
/// **提示：**
///
/// * `n == height.length`
/// * `2 <= n <= 10<sup>5</sup>`
/// * `0 <= height[i] <= 10<sup>4</sup>`

pub struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        // 双指针对应通的宽度
        // 当间距随着迭代缩小时，只有替换短边才有可能找到最大面积
        // 面积计算是 S = min(h_l, h_r) * 间距
        // 如果替换长边，则新面积为 S1 = min(h_l, h_r) * 新间距
        //
        // 由于新间距=间距-1
        // 所以 S1 < S
        // 因此，不应该替换长边，那么就移动短边

        let mut p_l = 0;
        let mut p_r = height.len() - 1;

        let mut max_capacity = 0;

        while p_l < p_r {
            // 计算当前面积
            let now_capacity = height[p_l].min(height[p_r]) * (p_r - p_l) as i32;
            max_capacity = max_capacity.max(now_capacity);

            // 移动短边
            if height[p_l] >= height[p_r] {
                p_r -= 1
            } else {
                p_l += 1
            }
        }
        max_capacity
    }
}

#[test]
fn test_max_area() {
    let height_1 = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
    assert_eq!(Solution::max_area(height_1), 49);

    let height_2 = vec![1, 1];
    assert_eq!(Solution::max_area(height_2), 1);
}
