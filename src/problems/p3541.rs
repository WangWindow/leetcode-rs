///! 3541. Find Most Frequent Vowel And Consonant
///! https://leetcode.com/problems/find-most-frequent-vowel-and-consonant/
use crate::Problem;
pub struct Solution;

impl Problem for Solution {
    const TITLE: &'static str =
        "找出出现频率最高的元音与辅音 (Find Most Frequent Vowel And Consonant)";
    fn run() {
        let s = "success".to_string();
        let result = Solution::max_freq_sum(s);
        println!("3541 => {}", result);
    }
}

impl Solution {
    pub fn max_freq_sum(s: String) -> i32 {
        // 统计元音字母和辅音字母的频率（暴力法）
        let mut vowel_count = [0; 26];

        let mut consonant_count = [0; 26];
        let vowels = "aeiou".chars().collect::<std::collections::HashSet<_>>();
        for ch in s.chars() {
            let idx = (ch as u8 - b'a') as usize;
            if vowels.contains(&ch) {
                vowel_count[idx] += 1;
            } else {
                consonant_count[idx] += 1;
            }
        }
        // 找出频率最高的元音字母和辅音字母
        let max_vowel = *vowel_count.iter().max().unwrap();
        let max_consonant = *consonant_count.iter().max().unwrap();
        max_vowel + max_consonant
    }
}
