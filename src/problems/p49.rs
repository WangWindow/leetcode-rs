/// 49. Group Anagrams 字母异位词分组
/// https://leetcode.com/problems/group-anagrams/
///
/// 给你一个字符串数组，请你将 字母异位词 组合在一起。可以按任意顺序返回结果列表。
///
/// **示例 1:**
///
/// **输入:** strs = ["eat", "tea", "tan", "ate", "nat", "bat"]
///
/// **输出:** [["bat"],["nat","tan"],["ate","eat","tea"]]
///
/// **解释：**
///
/// * 在 strs 中没有字符串可以通过重新排列来形成 `"bat"`。
/// * 字符串 `"nat"` 和 `"tan"` 是字母异位词，因为它们可以重新排列以形成彼此。
/// * 字符串 `"ate"` ，`"eat"` 和 `"tea"` 是字母异位词，因为它们可以重新排列以形成彼此。
///
/// **示例 2:**
///
/// **输入:** strs = [""]
///
/// **输出:** [[""]]
///
/// **示例 3:**
///
/// **输入:** strs = ["a"]
///
/// **输出:** [["a"]]
///
/// **提示：**
///
/// * `1 <= strs.length <= 10<sup>4</sup>`
/// * `0 <= strs[i].length <= 100`
/// * `strs[i]` 仅包含小写字母

pub struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        // 如果两个字符串排序后相同，则它们是字母异位词

        let mut map = std::collections::HashMap::new();

        for s in strs {
            let mut chars: Vec<char> = s.chars().collect();
            chars.sort_unstable();

            // 将排序后的字符串作为键（相当于分组标识）
            let key = chars.into_iter().collect::<String>();

            // 将原字符串添加到对应的分组中
            map.entry(key).or_insert_with(Vec::new).push(s);
        }

        // 将 HashMap 的值收集为 Vec<Vec<String>>
        map.into_values().collect()
    }
}
